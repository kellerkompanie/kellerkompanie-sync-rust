extern crate data_encoding;
extern crate pbr;



use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR;
use std::time::{Duration, Instant, SystemTime};

use glob::Pattern;
use pbr::ProgressBar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use walkdir::{DirEntry, WalkDir};

use settings::Settings;

mod hashing;
mod settings;
mod web_api;

fn extract_relative_filepath(absolute_filepath: &String) -> String {
    let index = absolute_filepath.find("@").unwrap_or(0);
    let (_, relative_filepath) = absolute_filepath.split_at(index);

    String::from(relative_filepath)
}

fn extract_addon_name(relative_filepath: &String) -> String {
    let index = relative_filepath.find(MAIN_SEPARATOR).unwrap_or(0);
    let (addon_name, _) = relative_filepath.split_at(index);

    String::from(addon_name)
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AddonIndex {
    name: String,
    files: Vec<FileIndex>,
}

impl AddonIndex {
    fn add_file_index(&mut self, value: FileIndex) {
        self.files.push(value);
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct FileIndex {
    relative_filepath: String,
    absolute_filepath: String,
    created: SystemTime,
    filesize: u64,
    hash: String,
}

const INDEX_FILE: &'static str = "index.json";

fn save_index(index: &HashMap<String, AddonIndex>) {
    let json = json!(index);

    let mut file = match File::create(INDEX_FILE) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the index file: {:?}", error)
        }
    };

    let json_string = match serde_json::to_string_pretty(&json) {
        Ok(json_string) => json_string,
        Err(error) => {
            panic!("Problem serializing index: {:?}", error)
        }
    };

    file.write_all(json_string.as_bytes()).expect("Something went wrong while writing the config file");
}

fn index_directory(directory: &str, index: &mut HashMap<String, AddonIndex>, settings: &Settings) {
    println!("Searching for files in {}", directory);

    let mut ignore_patterns = Vec::new();
    for pattern in &settings.ignore_files {
        let ignore_pattern = Pattern::new(pattern.as_str()).unwrap();
        ignore_patterns.push(ignore_pattern);
    }

    let start = Instant::now();
    let mut files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(directory).follow_links(settings.follow_links) {
        let dir_entry = entry.unwrap();

        if dir_entry.path().is_dir() {
            continue;
        }

        if settings.ignore_hidden && dir_entry.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }

        let mut ignore_file = false;
        for ignore_pattern in &ignore_patterns {
            if ignore_pattern.matches(dir_entry.file_name().to_str().unwrap()) {
                println!("Ignoring file {}", dir_entry.path().display());
                ignore_file = true;
                break;
            }
        }
        if ignore_file {
            continue;
        }

        files.push(dir_entry);
    }

    println!("Found {} files in {}", files.len(), directory);

    println!("Hashing files...");
    let mut pb = ProgressBar::new(files.len() as u64);
    pb.format("[=> ]");
    for dir_entry in files {
        let absolute_filepath = dir_entry.path().display().to_string();
        let relative_filepath = extract_relative_filepath(&absolute_filepath);
        let addon_name = extract_addon_name(&relative_filepath);

        if !index.contains_key(&addon_name) {
            let _addon_uuid = web_api::get_addon_uuid(&addon_name);

            index.insert(
                addon_name.clone(),
                AddonIndex {
                    name: addon_name.clone(),
                    files: Vec::new(),
                },
            );
        }

        let metadata = fs::metadata(dir_entry.path()).unwrap();
        let filesize = metadata.len();
        let created = metadata.created().unwrap();

        let hash = hashing::hash_file(&dir_entry);

        let file_index = FileIndex {
            relative_filepath,
            absolute_filepath,
            created,
            filesize,
            hash,
        };

        let addon_index = index.get_mut(&addon_name).unwrap();
        addon_index.add_file_index(file_index);

        pb.inc();
    }

    let duration: Duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


fn main() {
    println!("{:?}", web_api::get_addons());

    let settings = settings::load_settings();
    let mut index = HashMap::new();

    for directory in &settings.observed_directories {
        index_directory(directory, &mut index, &settings);
    }

    save_index(&index);
}
