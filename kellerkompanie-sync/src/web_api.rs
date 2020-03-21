extern crate reqwest;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::settings::{load_settings, Settings};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WebAddon {
    addon_name: String,
    addon_foldername: String,
    addon_uuid: String,
    addon_version: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WebAddonGroupInfo {
    addon_group_uuid: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WebAddonGroup {
    addon_group_name: String,
    addon_group_author: String,
    addon_group_uuid: String,
    addon_group_version: String,
    addons: Vec<WebAddon>,
}

pub fn get_addon_uuid(addon_name: &String, settings: &Settings) -> String {
    let mut addon_uuid_url = settings.api_url.clone();
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

pub fn get_addon_groups(settings: &Settings) -> Vec<WebAddonGroup> {
    let mut addon_groups_info_url = settings.api_url.clone();
    if !addon_groups_info_url.ends_with("/") {
        addon_groups_info_url = format!("{}{}", addon_groups_info_url, "/");
    }
    addon_groups_info_url = format!("{}{}", addon_groups_info_url, "addon_groups");

    let mut res = reqwest::get(&addon_groups_info_url).unwrap();
    let body = res.text().unwrap();

    let addon_group_infos: Vec<WebAddonGroupInfo> = match serde_json::from_str(&body) {
        Ok(uuid) => uuid,
        Err(error) => {
            panic!("Problem parsing the response: {:?}, the response was {:?}", error, body)
        }
    };

    let mut addon_groups_url = settings.api_url.clone();
    if !addon_groups_url.ends_with("/") {
        addon_groups_url = format!("{}{}", addon_groups_url, "/");
    }
    addon_groups_url = format!("{}{}", addon_groups_url, "addon_group/");

    let mut addon_groups: Vec<WebAddonGroup> = Vec::new();
    for addon_group_info in addon_group_infos {
        let addon_group_uuid = addon_group_info.addon_group_uuid;
        let mut res = reqwest::get(&format!("{}{}", addon_groups_url, addon_group_uuid)).unwrap();
        let body = res.text().unwrap();

        let addon_group: WebAddonGroup = match serde_json::from_str(&body) {
            Ok(uuid) => uuid,
            Err(error) => {
                panic!("Problem parsing the response: {:?}, the response was {:?}", error, body)
            }
        };

        addon_groups.push(addon_group);
    }

    addon_groups
}

pub fn update_addons(updated_addons: HashMap<String, String>) {
    let settings = load_settings();

    let mut addon_uuid_url = settings.api_url;
    if !addon_uuid_url.ends_with("/") {
        addon_uuid_url = format!("{}{}", addon_uuid_url, "/");
    }
    addon_uuid_url = format!("{}{}", addon_uuid_url, "update_addons");

    let params = [("updated_addons", format!("{}", json!(updated_addons)))];
    let client = reqwest::Client::new();
    let _response = match client.post(&addon_uuid_url)
        .form(&params)
        .send() {
        Ok(response) => response,
        Err(error) => {
            panic!("Problem updating updates through API: {:?}", error)
        }
    };
}
