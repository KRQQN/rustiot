use leptos::prelude::*;
use shared::dto::CurrentUser;

#[server(GetCurrentUser)]
pub async fn get_current_user() -> Result<CurrentUser, ServerFnError> {
    let user = use_context::<CurrentUser>().ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    Ok(user)
}
