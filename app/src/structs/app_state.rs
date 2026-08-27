use leptos::config::LeptosOptions;

#[cfg(feature = "ssr")]
use {axum::extract::FromRef, influxdb3_client::Client, sqlx::PgPool, std::sync::Arc};

#[cfg_attr(feature = "ssr", derive(FromRef))]
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,

    #[cfg(feature = "ssr")]
    pub pg_pool: PgPool,

    #[cfg(feature = "ssr")]
    pub influx: Arc<Client>,
}
