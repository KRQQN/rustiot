mod auth;

use crate::auth::{middleware::auth_middleware, protected_pages::protected_pages};
use app::*;
use axum::{body::Body, http::Request, middleware, routing::post, Router};
use influxdb3_client::{Client, ClientConfig};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use shared::dto::CurrentUser;
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .test_before_acquire(true)
        .connect("postgres://leptos:leptos@localhost:5432/leptos")
        .await?;

    let influx = {
        let host = std::env::var("INFLUX_HOST").unwrap_or_else(|_| "http://localhost:8181".into());
        let token = std::env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN must be set");
        let database = std::env::var("INFLUX_DATABASE").unwrap_or_else(|_| "yolo".into());

        Client::new(
            ClientConfig::builder()
                .host(&host)
                .token(&token)
                .database(&database)
                .build()?,
        )
        .await?
    };

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pg_pool,
        influx: Arc::new(influx),
    };

    let app = Router::new()
        .route(
            "/api/{*fn_name}",
            post({
                let app_state = app_state.clone();
                move |req: Request<Body>| {
                    let current_user = req.extensions().get::<CurrentUser>().cloned();

                    handle_server_fns_with_context(
                        {
                            let app_state = app_state.clone();
                            move || provide_context(app_state.clone())
                        },
                        req,
                    )
                }
            }),
        )
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || {
                    provide_context(app_state.clone());
                }
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .layer(middleware::from_fn(protected_pages))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .layer(CookieManagerLayer::new())
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
