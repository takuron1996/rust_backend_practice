use envsubst;
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::env;
use std::process;

/// ${val}を環境変数の値に置換
pub fn deserialize_with_env<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(substitute_env_variables(&s))
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{(\w+)\}").unwrap());

fn substitute_env_variables(input: &str) -> String {
    if envsubst::is_templated(input) {
        let mut context = std::collections::HashMap::new();
        for (_, [cap]) in RE.captures_iter(input).map(|c| c.extract()) {
            match env::var(cap) {
                Ok(val) => context.insert(cap.to_string(), val),
                Err(e) => {
                    error!("{}", e);
                    process::exit(1);
                }
            };
        }
        envsubst::substitute(input, &context).unwrap()
    } else {
        input.to_string()
    }
}
