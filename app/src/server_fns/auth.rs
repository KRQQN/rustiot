#[cfg(feature = "ssr")]
use axum::Extension;

#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use shared::dto::CurrentUser;

#[cfg(feature = "ssr")]
pub async fn current_user() -> Result<CurrentUser, ServerFnError> {
    let Extension(user) = leptos_axum::extract::<Extension<CurrentUser>>()
        .await
        .map_err(|_| ServerFnError::new("Not authenticated"))?;

    Ok(user)
}
