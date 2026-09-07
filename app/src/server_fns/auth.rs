#[cfg(feature = "ssr")]
use axum::Extension;

use leptos::prelude::*;
use shared::dto::CurrentUser;

#[cfg(feature = "ssr")]
pub async fn current_user() -> Result<CurrentUser, ServerFnError> {
    let Extension(user) = leptos_axum::extract::<Extension<CurrentUser>>()
        .await
        .map_err(|_| ServerFnError::new("Not authenticated"))?;

    Ok(user)
}
