use serde::{Serialize, Deserialize};
use std::fs;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    ip_address: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip_address: String::from("127.0.0.1"),
            port: 8080,
            debug: true,
        }
    }
}

fn load_or_create_config() -> Config {
    let path = "config.toml";

    if let Ok(text) = fs::read_to_string(path) {
        toml::from_str(&text).unwrap_or_default()
    } else {
        let default_cfg = Config::default();
        let text = toml::to_string_pretty(&default_cfg).unwrap();
        fs::write(path, text).unwrap();
        default_cfg
    }
}

pub fn toml_audit() {
    let cfg = load_or_create_config();
    println!("Config: {:?}", cfg);
}
