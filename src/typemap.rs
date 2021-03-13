use serenity::{model::id::UserId, prelude::TypeMapKey};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{config::Config, shared_cache::SharedCache};

pub struct TypeMapConfig;
pub struct TypeMapSharedCache;
pub struct TypeMapVerificationCodes;

impl TypeMapKey for TypeMapConfig {
    type Value = Config;
}

impl TypeMapKey for TypeMapSharedCache {
    type Value = SharedCache;
}

impl TypeMapKey for TypeMapVerificationCodes {
    type Value = Arc<Mutex<HashMap<String, (UserId, String)>>>;
}
