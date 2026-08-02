use crate::server;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;

#[server(Login)]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};
        use leptos_axum::extract;
        use sqlx;
        use tower_cookies::{Cookie, Cookies};
        use uuid::Uuid;

        use crate::structs::app_state::AppState;

        if email.trim().is_empty() {
            return Err(ServerFnError::new("Email is required"));
        }
        if password.trim().is_empty() {
            return Err(ServerFnError::new("Password is required"));
        }

        let app_state = expect_context::<AppState>();

        let row: Option<(Uuid, String)> =
            sqlx::query_as("SELECT id, password_hash FROM users WHERE email = $1")
                .bind(&email)
                .fetch_optional(&app_state.pg_pool)
                .await?;

        let Some((user_id, password_hash)) = row else {
            return Err(ServerFnError::new("Invalid email or password"));
        };

        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|_| ServerFnError::new("Invalid email or password"))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ServerFnError::new("Invalid email or password"))?;

        let token = shared::jwt::create(user_id, &email)
            .map_err(|_| ServerFnError::new("Failed to generate JWT"))?;
        let cookies: Cookies = extract().await?;

        let mut cookie = Cookie::new("session", token);
        cookie.set_http_only(true);
        cookie.set_secure(false);
        cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
        cookie.set_path("/");
        cookie.set_max_age(tower_cookies::cookie::time::Duration::days(7));

        cookies.add(cookie);

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password);
        unreachable!()
    }
}
