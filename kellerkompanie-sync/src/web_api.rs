extern crate reqwest;

use std::collections::{HashMap};

use serde::{Deserialize, Serialize};
use serde_json::json;

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

pub fn get_addon_uuid(addon_name: &String) -> String {
    let settings = load_settings();

    let mut addon_uuid_url = settings.api_url;
    if !addon_uuid_url.ends_with("/") {
        addon_uuid_url = format!("{}{}", addon_uuid_url, "/");
    }
    addon_uuid_url = format!("{}{}{}", addon_uuid_url, "addon/", addon_name);

    let mut res = reqwest::get(&addon_uuid_url).unwrap();
    let body = res.text().unwrap();

    let result: HashMap<String, String> = match serde_json::from_str(&body) {
        Ok(uuid) => uuid,
        Err(error) => {
            panic!("Problem parsing the response: {:?}, the response was {:?}", error, body)
        }
    };

    result.get("uuid").unwrap().clone()
}

pub fn update_addons(updated_addons: HashMap<String, String>) {
    let settings = load_settings();

    let mut addon_uuid_url = settings.api_url;
    if !addon_uuid_url.ends_with("/") {
        addon_uuid_url = format!("{}{}", addon_uuid_url, "/");
    }
    addon_uuid_url = format!("{}{}", addon_uuid_url, "update_addons");

    let client = reqwest::Client::new();
    let _response = match client.post(&addon_uuid_url)
        .body(format!("{}", json!(updated_addons)))
        .send() {
        Ok(response) => response,
        Err(error) => {
            panic!("Problem updating updates through API: {:?}", error)
        }
    };
}
