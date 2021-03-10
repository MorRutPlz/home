use serenity::prelude::TypeMapKey;

use crate::{config::Config, shared_cache::SharedCache};

pub struct TypeMapConfig;
pub struct TypeMapSharedCache;

impl TypeMapKey for TypeMapConfig {
    type Value = Config;
}

impl TypeMapKey for TypeMapSharedCache {
    type Value = SharedCache;
}
