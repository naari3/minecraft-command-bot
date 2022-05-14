use std::sync::Arc;

use serenity::prelude::TypeMapKey;

use crate::domains::send_rule::SendRule;

pub struct SendRules;

impl TypeMapKey for SendRules {
    type Value = Arc<Vec<Box<dyn SendRule + Send + Sync>>>;
}
