use crate::server;
use leptos::server_fn::ServerFnError;

#[server(Register)]
pub async fn register(email: String, password: String) -> Result<(), ServerFnError> {
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHasher,
    };
    use leptos::prelude::*;
    use sqlx::{query, query_as, PgPool};
    use uuid::Uuid;

    if email.trim().is_empty() {
        return Err(ServerFnError::new("Email is required"));
    }
    if password.len() < 8 {
        return Err(ServerFnError::new("Password must be at least 8 characters"));
    }

    let pool = expect_context::<PgPool>();

    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_optional(&pool)
        .await?;

    if exists.is_some() {
        return Err(ServerFnError::new("Email already registered"));
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ServerFnError::new("Failed to hash password"))?
        .to_string();

    let user_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(password_hash)
    .execute(&pool)
    .await?;

    Ok(())
}
