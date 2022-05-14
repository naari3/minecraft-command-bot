use crate::minecraft_line::MinecraftLine;

pub mod advancement_rule;
pub mod chat_rule;
pub mod death_rule;
pub mod login_rule;
pub mod rcon_rule;
pub mod server_rule;

pub trait SendRule {
    fn send(&self, line: &MinecraftLine) -> Option<String>;
}
