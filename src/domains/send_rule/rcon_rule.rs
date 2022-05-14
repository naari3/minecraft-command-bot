use crate::minecraft_line::MinecraftLine;
use once_cell::sync::Lazy;
use regex::Regex;

use super::SendRule;

#[derive(Clone)]
pub struct RconRule;

static RCON_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[(.*?)]\s(.*)$").unwrap());

impl SendRule for RconRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        RCON_RE.captures(&line.message).map(|cap| {
            let name = cap
                .get(1)
                .map(|name| name.as_str().to_string())
                .unwrap_or("".to_string());
            let message = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());
            format!("**[{name}]**: {message}").to_string()
        })
    }
}
