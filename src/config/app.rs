use crate::config::model::{ Config };

pub fn get_config() -> anyhow::Result<Config> {
    let mut config: Config = Config::new();
    config.update();
    Ok(config)
}
