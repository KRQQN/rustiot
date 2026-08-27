use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceReading {
    pub time: String,
    pub device_id: String,
    pub humidity: f64,
    pub temperature: f64,
}

#[server(GetDeviceData)]
pub async fn get_device_data(
    device_id: String,
    limit: Option<u32>,
) -> Result<Vec<DeviceReading>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::structs::app_state::AppState;

        let app_state = expect_context::<AppState>();
        let client = &app_state.influx;

        let limit = limit.unwrap_or(100);

        let result = client
            .sql(
                "SELECT humidity, device_id, time, temperature \
         FROM telemetry \
         WHERE time >= now() - interval '2 days' \
         AND device_id = $device_id \
         ORDER BY time DESC \
         LIMIT $limit",
            )
            .param("device_id", device_id.as_str())
            .param("limit", limit as i64)
            .await
            .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

        let mut readings = Vec::new();

        for row in result {
            let row =
                row.map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

            readings.push(DeviceReading {
                time: row["time"].as_str().unwrap_or_default().to_string(),
                device_id: row["device_id"].as_str().unwrap_or_default().to_string(),
                humidity: row["humidity"].as_f64().unwrap_or(0.0),
                temperature: row["temperature"].as_f64().unwrap_or(0.0),
            });
        }

        Ok(readings)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
