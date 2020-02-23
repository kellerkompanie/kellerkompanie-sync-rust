use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Settings {
    pub(crate) api_url: String,
    pub(crate) observed_directories: Vec<String>,
    pub(crate) follow_links: bool,
    pub(crate) ignore_hidden: bool,
    pub(crate) ignore_files: Vec<String>,
}

const CONFIG_FILE: &'static str = "config.json";

pub fn save_settings(settings: &Settings) {
    let json = json!(settings);

    let mut file = match File::create(CONFIG_FILE) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the config file: {:?}", error)
        }
    };

    let json_string = match serde_json::to_string_pretty(&json) {
        Ok(json_string) => json_string,
        Err(error) => {
            panic!("Problem serializing settings: {:?}", error)
        }
    };

    file.write_all(json_string.as_bytes()).expect("Something went wrong while writing the config file");
}

pub fn load_settings() -> Settings {
    let settings: Settings;
    let path = Path::new(CONFIG_FILE);
    if path.exists() {
        let contents =  match fs::read_to_string(CONFIG_FILE) {
            Ok(contents) => contents,
            Err(error) => {
                panic!("Problem reading the config file: {:?}", error)
            }
        };
        settings = match serde_json::from_str(&contents) {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem parsing the config file: {:?}", error)
            }
        };
    } else {
        settings = Settings {
            api_url: String::from("https://localhost:5000/"),
            observed_directories: Vec::new(),
            follow_links: false,
            ignore_hidden: false,
            ignore_files: Vec::new(),
        };
        save_settings(&settings);
    }

    settings
}