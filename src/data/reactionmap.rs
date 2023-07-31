use serenity::{model::channel::ReactionType, model::id::RoleId, prelude::TypeMapKey};
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct ReactionMap;

impl TypeMapKey for ReactionMap {
    type Value = Arc<RwLock<Vec<(ReactionType, RoleId)>>>;
}
