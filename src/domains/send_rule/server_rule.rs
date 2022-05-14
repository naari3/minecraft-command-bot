use crate::minecraft_line::MinecraftLine;
use regex::Regex;

use super::SendRule;

#[derive(Clone)]
pub struct ServerRule;

impl SendRule for ServerRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        let stopped_re = Regex::new(r"^Stopping\sserver$").unwrap();

        if stopped_re.is_match(&line.message)
            || stopped_re.is_match(&line.message)
            || stopped_re.is_match(&line.message)
        {
            return Some(format!("**{}**", line.message).to_string());
        }
        None
    }
}
