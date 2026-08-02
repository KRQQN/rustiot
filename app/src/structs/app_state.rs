use leptos::config::LeptosOptions;

#[cfg(feature = "ssr")]
use axum::extract::FromRef;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg_attr(feature = "ssr", derive(FromRef))]
#[derive(Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,

    #[cfg(feature = "ssr")]
    pub pg_pool: PgPool,
}
