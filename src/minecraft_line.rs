#[derive(Debug, Clone)]
pub struct MinecraftLine {
    pub time: String,
    pub caused_at: String,
    pub level: String,
    pub message: String,
}

impl MinecraftLine {
    pub fn new(time: String, caused_at: String, level: String, message: String) -> Self {
        Self {
            time,
            caused_at,
            level,
            message,
        }
    }
}
