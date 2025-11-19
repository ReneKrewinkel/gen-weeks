// use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::{Deserialize, Serialize };
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: Option<String>,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            version: None,
            author: None,
            homepage: None,
            description: None,
            repository: None
        }
    }
    pub fn update(&mut self) {
        self.version = option_env!("CARGO_PKG_VERSION").map(|s| s.to_string());
        self.author = option_env!("CARGO_PKG_AUTHORS").map(|s| s.to_string());
        self.homepage = option_env!("CARGO_PKG_HOMEPAGE").map(|s| s.to_string());
        self.description = option_env!("CARGO_PKG_DESCRIPTION").map(|s| s.to_string());
        self.repository = option_env!("CARGO_PKG_REPOSITORY").map(|s| s.to_string());
    }

    // pub fn get_config_item(&self, key: &str) -> String {
    //     match key {
    //         "version" => self.version.clone().unwrap_or_default(),
    //         "author" => self.author.clone().unwrap_or_default(),
    //         "homepage" => self.homepage.clone().unwrap_or_default(),
    //         "description" => self.description.clone().unwrap_or_default(),
    //         "repository" => self.repository.clone().unwrap_or_default(),
    //         _ => panic!("Invalid config key"),
    //     }
    // }

    pub fn get_config(&self) -> (String, String, String, String, String)  {
        (
            self.version.clone().unwrap_or_default(),
            self.author.clone().unwrap_or_default(),
            self.homepage.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
            self.repository.clone().unwrap_or_default()
        )
    }
}


