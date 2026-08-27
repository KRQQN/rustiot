use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use influxdb3_client::{Client, ClientConfig};

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
        let host =
            std::env::var("INFLUX_HOST").unwrap_or_else(|_| "http://localhost:8181".to_string());
        let token = std::env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN must be set");
        let database =
            std::env::var("INFLUX_DATABASE").unwrap_or_else(|_| "your_database".to_string());

        let client = Client::new(
            ClientConfig::builder()
                .host(&host)
                .token(&token)
                .database(&database)
                .build()
                .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?,
        )
        .await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

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
        // This branch is never executed on the client
        unreachable!()
    }
}
