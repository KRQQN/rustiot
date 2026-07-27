pub enum Topic<'a> {
    DeviceTelemetry { device_id: &'a str },
    DeviceStatus { device_id: &'a str },
    Unknown,
}

impl<'a> Topic<'a> {
    pub fn parse(topic: &'a str) -> Self {
        let parts: Vec<_> = topic.split('/').collect();

        match parts.as_slice() {
            ["devices", id, "telemetry"] => Self::DeviceTelemetry { device_id: id },

            ["devices", id, "status"] => Self::DeviceStatus { device_id: id },

            _ => Self::Unknown,
        }
    }
}
