use serenity::prelude::TypeMapKey;
use std::path::PathBuf;

pub(crate) struct BotMap;

impl TypeMapKey for BotMap {
    type Value = PathBuf;
}
