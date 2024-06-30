use common::config::env::deserialize_with_env;
use confy::{self, ConfyError};
use log::error;
use serde::{Deserialize, Serialize};
use std::process;
/// CORS関連
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CorsConfig {
    /// CORSで許可するオリジン
    #[serde(deserialize_with = "deserialize_with_env")]
    pub allowed_origin: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub cors: CorsConfig,
}

/// YAMLからコンフィグの値を取得
pub fn get_config() -> Config {
    let cfg: Result<Config, ConfyError> = confy::load_path("web/src/config/config.yml");
    match cfg {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    }
}
