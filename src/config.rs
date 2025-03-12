use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    pub osu: OsuConfig,
}

impl Config {
    pub fn parse() -> Self {
        let config_path = crate::SELF_DIR.join(CONFIG_FILENAME);

        if config_path.exists() {
            let mut file = File::open(config_path).unwrap();

            let mut content = String::new();
            file.read_to_string(&mut content)
                .unwrap();

            toml::from_str(&content).unwrap()
        } else {
            println!("No config file found! Creating default config...");
            Self::create_default(config_path);
            std::process::exit(0);
        }
    }

    fn create_default(file_path: PathBuf) {
        let default_config = Self {
            server: ServerConfig::default(),
            osu: OsuConfig {
                client_id: 0,
                client_secret: String::from(
                    "Get your client id and secret from https://osu.ppy.sh/home/account/edit#oauth",
                ),
            },
        };
        let toml_string = toml::to_string_pretty(&default_config).unwrap();
        let mut file = File::create(file_path).unwrap();
        file.write(toml_string.as_bytes())
            .unwrap();
    }
}

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub listener: ListenerConfig,
    pub cache: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listener: ListenerConfig {
                address: String::from("127.0.0.1"),
                port: 7270,
            },
            cache: true,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListenerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct OsuConfig {
    pub client_id: u64,
    pub client_secret: String,
}
