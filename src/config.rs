use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::ArcSwap;
use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serenity::{all::*, prelude::TypeMapKey};
use snafu::{OptionExt, ResultExt};

use crate::error::BotError;

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BotCfg {
    pub time_offset: i32,
    pub token: String,
    #[serde(skip)]
    pub path: PathBuf,
}

impl TypeMapKey for BotCfg {
    type Value = Arc<ArcSwap<BotCfg>>;
}

#[allow(dead_code)]
pub(crate) trait GetCfg {
    async fn cfg(&self) -> Result<Arc<ArcSwap<BotCfg>>, BotError>;
}

impl GetCfg for Context {
    async fn cfg(&self) -> Result<Arc<ArcSwap<BotCfg>>, BotError> {
        self.data
            .read()
            .await
            .get::<BotCfg>()
            .cloned()
            .whatever_context("Failed to get bot configuration from type map")
    }
}

impl BotCfg {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, BotError> {
        Ok(Self {
            path: path.as_ref().to_owned(),
            ..Figment::new()
                .merge(Json::file(path))
                .merge(Env::prefixed("DOG_BOT_"))
                .extract_lossy()
                .whatever_context::<&str, BotError>("Failed to read bot configuration")?
        })
    }

    pub fn write(&self) -> Result<(), BotError> {
        let json = serenity::json::to_string_pretty(self)
            .whatever_context::<&str, BotError>("Failed to serialize configuration to JSON")?;
        std::fs::write(&self.path, json).whatever_context("Failed to write configuration file")
    }
}
