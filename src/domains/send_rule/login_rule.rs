use crate::minecraft_line::MinecraftLine;
use regex::Regex;

use super::SendRule;

#[derive(Clone)]
pub struct LoginRule;

impl SendRule for LoginRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        let join_re = Regex::new(r"^(.*)\s(joined|left)\sthe\sgame$").unwrap();
        join_re.captures(&line.message).map(|cap| {
            let name = cap
                .get(1)
                .map(|name| name.as_str().to_string())
                .unwrap_or("".to_string());
            let kind = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());
            format!("**{name}** {kind} the game").to_string()
        })
    }
}
