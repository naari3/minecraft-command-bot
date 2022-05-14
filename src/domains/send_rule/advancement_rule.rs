use crate::minecraft_line::MinecraftLine;
use once_cell::sync::Lazy;
use regex::Regex;

use super::SendRule;

static ADVANCEMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(.*)\s\shas\smade\sthe\sadvancement\s\[(.*)\]$").unwrap());

#[derive(Clone)]
pub struct AdvancementRule;

impl SendRule for AdvancementRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        ADVANCEMENT_RE.captures(&line.message).map(|cap| {
            let name = cap
                .get(1)
                .map(|name| name.as_str().to_string())
                .unwrap_or("".to_string());
            let kind = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());
            format!("**{name}** has made the advancement _**{kind}**_").to_string()
        })
    }
}
