use leptos::prelude::*;

#[server(Register)]
pub async fn register(email: String, password: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };
        use leptos::prelude::*;
        use sqlx::query_as;
        use uuid::Uuid;

        use crate::AppState;

        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err(ServerFnError::new("Email is required"));
        }

        if password.len() < 8 {
            return Err(ServerFnError::new("Password must be at least 8 characters"));
        }

        let app_state = expect_context::<AppState>();

        let exists: Option<(Uuid,)> = query_as("SELECT id FROM users WHERE email = $1")
            .bind(&email)
            .fetch_optional(&app_state.pg_pool)
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
        .bind(&email)
        .bind(password_hash)
        .execute(&app_state.pg_pool)
        .await?;

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password);
        unreachable!()
    }
}
