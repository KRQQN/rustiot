use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryMessage {
    //pub msg_type: String,
    pub data: TemperatureHumidity,
}

#[derive(Debug, Deserialize)]
pub struct TemperatureHumidity {
    pub temp: f64,
    pub hum: f64,
}

#[derive(Debug, Deserialize)]
pub struct StatusMessage {
    pub status: String,
    pub value: String,
}
