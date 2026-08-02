mod auth;

use crate::auth::middleware::auth_middleware;
use app::*;
use axum::{body::Body, http::Request, middleware, routing::post, Router};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
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

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pg_pool,
    };

    let app = Router::new()
        .route(
            "/api/{*fn_name}",
            post({
                let app_state = app_state.clone();
                move |req: Request<Body>| {
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
