use crate::minecraft_line::MinecraftLine;
use regex::Regex;

use super::SendRule;

#[derive(Clone)]
pub struct ChatRule;

impl SendRule for ChatRule {
    fn allow_send(&self, line: &MinecraftLine) -> bool {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return false;
        }
        let chat_re = Regex::new(r"^<(.*?)>\s(.*)$").unwrap();
        chat_re.captures(&line.message).is_some()
    }

    fn send(&self, line: &MinecraftLine) -> Option<String> {
        let chat_re = Regex::new(r"^<(.*?)>\s(.*)$").unwrap();
        chat_re.captures(&line.message).map(|cap| {
            let name = cap
                .get(1)
                .map(|name| name.as_str().to_string())
                .unwrap_or("".to_string());
            let message = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());
            format!("**{name}**: {message}").to_string()
        })
    }
}
