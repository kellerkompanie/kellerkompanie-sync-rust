extern crate reqwest;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::settings::load_settings;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AddonInfo {
    addon_name: String,
    addon_foldername: String,
    addon_id: u64,
    addon_uuid: String,
    addon_version: String,
}

pub fn get_addons() -> Vec<AddonInfo> {
    let settings = load_settings();

    let mut addons_url = settings.api_url;
    if !addons_url.ends_with("/") {
        addons_url = format!("{}{}", addons_url, "/");
    }
    addons_url = format!("{}{}", addons_url, "addons");
    println!("requesting to {}", addons_url);
    let mut res = reqwest::get(&addons_url).unwrap();
    let body = res.text().unwrap();

    let addons: Vec<AddonInfo> = match serde_json::from_str(&body) {
        Ok(addons) => addons,
        Err(error) => {
            panic!("Problem parsing the config file: {:?}", error)
        }
    };

    addons
}

pub fn get_addon_uuid(addon_name: &String) -> String {
    let settings = load_settings();

    let mut addon_uuid_url = settings.api_url;
    if !addon_uuid_url.ends_with("/") {
        addon_uuid_url = format!("{}{}", addon_uuid_url, "/");
    }
    addon_uuid_url = format!("{}{}{}", addon_uuid_url, "addon/", addon_name);
    println!("requesting to {}", addon_uuid_url);

    let mut res = reqwest::get(&addon_uuid_url).unwrap();
    let body = res.text().unwrap();

    let result: HashMap<String, String> = match serde_json::from_str(&body) {
        Ok(uuid) => uuid,
        Err(error) => {
            panic!("Problem parsing the response: {:?}", error)
        }
    };

    result.get("uuid").unwrap().clone()
}
