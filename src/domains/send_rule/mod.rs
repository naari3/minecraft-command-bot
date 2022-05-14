use crate::minecraft_line::MinecraftLine;

pub mod chat_rule;
pub mod login_rule;
pub mod rcon_rule;

pub trait SendRule: SendRuleClone {
    fn allow_send(&self, line: &MinecraftLine) -> bool;
    fn send(&self, line: &MinecraftLine) -> Option<String>;
}

pub trait SendRuleClone {
    fn clone_box(&self) -> Box<dyn SendRule>;
}

impl<T> SendRuleClone for T
where
    T: SendRule + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn SendRule> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SendRule> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
