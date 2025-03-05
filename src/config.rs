use serde::{Deserialize, Serialize};
use std::{
    env::current_exe,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub listener: ListenerConfig,
    pub osu: OsuConfig,
}

impl Config {
    pub fn parse() -> Self {
        let config_dir = Self::config_dir();
        if config_dir.exists() {
            let mut file = File::open(config_dir).unwrap();

            let mut content = String::new();
            file.read_to_string(&mut content)
                .unwrap();

            toml::from_str(&content).unwrap()
        } else {
            println!("No config file found! Creating default config...");
            Self::create_default();
            std::process::exit(0);
        }
    }

    fn config_dir() -> PathBuf {
        let exe_path = current_exe().unwrap();
        let parent = exe_path
            .parent()
            .unwrap();
        parent.join(CONFIG_FILENAME)
    }

    fn create_default() {
        let default_config = Self {
            listener: ListenerConfig::default(),
            osu: OsuConfig {
                client_id: 0,
                client_secret: String::from(
                    "Get your client id and secret from https://osu.ppy.sh/home/account/edit#oauth",
                ),
            },
        };
        let toml_string = toml::to_string_pretty(&default_config).unwrap();
        let mut file = File::create(Self::config_dir()).unwrap();
        file.write_all(toml_string.as_bytes())
            .unwrap();
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListenerConfig {
    pub address: String,
    pub port: u16,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 7270,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct OsuConfig {
    pub client_id: u64,
    pub client_secret: String,
}
