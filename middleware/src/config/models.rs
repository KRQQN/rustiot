use std::time::Duration;

pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub keep_alive: Duration,
}

pub struct InfluxConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub token: String,
}
