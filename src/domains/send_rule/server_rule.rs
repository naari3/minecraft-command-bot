use crate::minecraft_line::MinecraftLine;
use once_cell::sync::Lazy;
use regex::Regex;

use super::SendRule;

static SERVER_REGEXS: Lazy<Vec<Regex>> = Lazy::new(|| {
    let res = vec![
        r"^Starting\sminecraft\sserver\sversion\s.*$",
        r"^Stopping\sserver$",
        r"^Done\s\(.*s\)!",
    ];
    res.into_iter().map(|re| Regex::new(re).unwrap()).collect()
});

#[derive(Clone)]
pub struct ServerRule;

impl SendRule for ServerRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        for re in SERVER_REGEXS.iter() {
            if re.is_match(&line.message) {
                return Some(format!("**{}**", line.message.clone()));
            }
        }
        None
    }
}
