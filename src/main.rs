use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::input::*;
use serde::{Deserialize, Serialize};
use std::fs;

use dialog::*;
mod dialog;

#[derive(Serialize, Deserialize, Debug)]
struct Configs {
    config: ConfigDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigDetails {
    asset_dir: String,
    fontmap: HashMap<String, String>,
}


#[macroquad::main("UI showcase")]
async fn main() {
    let mut positionhash: HashMap<usize, Vec2> = HashMap::new();
    let configs = load_config("config.json").await;

    info!("Asset Directory: {}", &configs.asset_dir);

    info!("Fonts:");
    for (key, value) in &configs.fontmap {
        info!("Font ID: {}, File: {}", key, value);
    }

    run_dialog(&configs).await;

    loop {
    }
}

async fn load_config(file_path: &str) -> ConfigDetails {
    let file_content = fs::read_to_string(file_path).expect("CANNOT READ: CONFIG");
    let configs: Configs = serde_json::from_str(&file_content).expect("CANNOT PARSE: CONFIG");

    configs.config
}
