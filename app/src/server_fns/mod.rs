#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use std::sync::OnceLock;

#[cfg(feature = "ssr")]
static DB_POOL: OnceLock<PgPool> = OnceLock::new();

#[cfg(feature = "ssr")]
pub fn init_pool(pool: PgPool) {
    DB_POOL.set(pool).expect("pool already initialized");
}

#[cfg(feature = "ssr")]
pub fn pool() -> &'static PgPool {
    DB_POOL.get().expect("pool not initialized")
}

pub mod auth;
pub mod devices;
pub mod get_current_user;
pub mod get_device_data;
pub mod login;
pub mod register;
