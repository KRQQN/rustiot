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
    timespan: String,
    limit: Option<u32>,
) -> Result<Vec<DeviceReading>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::structs::app_state::AppState;

        let app_state = expect_context::<AppState>();
        let client = &app_state.influx;
        let limit = limit.unwrap_or(500);

        let interval = match timespan.as_str() {
            "24h" => "1 day",
            "7d" => "7 days",
            "30d" => "30 days",
            "90d" => "90 days",
            _ => "7 days",
        };

        let sql = format!(
            "SELECT humidity, device_id, time, temperature \
             FROM telemetry \
             WHERE time >= now() - interval '{interval}' \
             AND device_id = $device_id \
             ORDER BY time ASC \
             LIMIT $limit"
        );

        let result = client
            .sql(&sql)
            .param("device_id", device_id.as_str())
            .param("limit", limit as i64)
            .await
            .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

        let mut readings = Vec::new();
        for row in result {
            let row =
                row.map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

            // Debug once if needed:
            leptos::logging::log!("raw time = {:?}", row["time"]);

            let time = cell_to_time_string(&row["time"]);

            readings.push(DeviceReading {
                time,
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

#[cfg(feature = "ssr")]
use influxdb3_client::Value;

#[cfg(feature = "ssr")]
fn cell_to_time_string(cell: &Value) -> String {
    use chrono::{TimeZone, Utc};

    if let Some(s) = cell.as_str() {
        return s.to_string();
    }

    // nanosecond epoch (most common for time)
    if let Some(ns) = cell.as_i64() {
        let secs = ns / 1_000_000_000;
        let nsecs = (ns % 1_000_000_000) as u32;
        if let Some(dt) = Utc.timestamp_opt(secs, nsecs).single() {
            return dt.to_rfc3339();
        }
    }

    // Debug fallback: Timestamp(1787904244266127876)
    let debug = format!("{cell:?}");
    if let Some(start) = debug.find('(') {
        if let Some(end) = debug.find(')') {
            if let Ok(ns) = debug[start + 1..end].parse::<i64>() {
                let secs = ns / 1_000_000_000;
                let nsecs = (ns % 1_000_000_000) as u32;
                if let Some(dt) = Utc.timestamp_opt(secs, nsecs).single() {
                    return dt.to_rfc3339();
                }
            }
        }
    }

    debug
}
