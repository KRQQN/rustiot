use leptos::prelude::*;

use crate::{components::devices::device_card::Device, AppState};

#[cfg(feature = "ssr")]
use crate::server_fns::auth::current_user;
use uuid::Uuid;

#[server(GetDevices)]
pub async fn get_devices() -> Result<Vec<Device>, ServerFnError> {
    let app_state = expect_context::<AppState>();
    let user = current_user().await?;

    let devices = sqlx::query_as!(
        Device,
        r#"
            SELECT
                id,
                name,
                device_type,
                protocol,
                is_active,
                last_seen_at
            FROM devices
            WHERE user_id = $1
            ORDER BY name
        "#,
        user.id
    )
    .fetch_all(&app_state.pg_pool)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(devices)
}

#[server(AddDevice)]
pub async fn add_device(
    name: String,
    device_type: String,
    config: serde_json::Value,
) -> Result<Device, ServerFnError> {
    let app_state = expect_context::<AppState>();

    let user = current_user().await?;

    let device = sqlx::query_as!(
        Device,
        r#"
            INSERT INTO devices (
                user_id,
                name,
                device_type,
                protocol,
                config
            )
            VALUES ($1, $2, $3, 'http', $4)
            RETURNING
                id,
                name,
                device_type,
                protocol,
                is_active,
                last_seen_at
        "#,
        user.id,
        name,
        device_type,
        config
    )
    .fetch_one(&app_state.pg_pool)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(device)
}

#[server(RemoveDevice)]
pub async fn remove_device(id: Uuid) -> Result<(), ServerFnError> {
    let app_state = expect_context::<AppState>();
    let user = current_user().await?;

    let result = sqlx::query!(
        r#"
            DELETE FROM devices
            WHERE id = $1 AND user_id = $2
        "#,
        id,
        user.id
    )
    .execute(&app_state.pg_pool)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new("Device not found"));
    }

    Ok(())
}
