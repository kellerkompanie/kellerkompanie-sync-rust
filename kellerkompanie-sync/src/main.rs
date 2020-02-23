extern crate chrono;
extern crate data_encoding;
extern crate pbr;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::{MAIN_SEPARATOR, Path};
use std::time::{Duration, Instant, SystemTime};

use chrono::{DateTime, Utc};
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
pub struct LocalAddon {
    name: String,
    uuid: String,
    version: String,
    files: HashMap<String, FileIndex>,
}

impl LocalAddon {
    fn add_file_index(&mut self, file_index: FileIndex) {
        self.files.insert(file_index.absolute_filepath.clone(), file_index);
    }
}

impl Hash for LocalAddon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.uuid.hash(state);
    }
}

impl PartialEq for LocalAddon {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.uuid == other.uuid
    }
}

impl Eq for LocalAddon {}

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
const CACHE_FILE: &'static str = "filecache.json";

fn load_filecache() -> HashMap<String, LocalAddon> {
    let index: HashMap<String, LocalAddon>;
    let path = Path::new(CACHE_FILE);
    if path.exists() {
        let contents = match fs::read_to_string(CACHE_FILE) {
            Ok(contents) => contents,
            Err(error) => {
                panic!("Problem reading the config file: {:?}", error)
            }
        };
        index = match serde_json::from_str(&contents) {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem parsing the config file: {:?}", error)
            }
        };
    } else {
        index = HashMap::new();
    }

    index
}

fn save_filecache(index: FilesCache) {
    let json = json!(index.map);

    let mut file = match File::create(CACHE_FILE) {
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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Addon {
    addon_name: String,
    addon_uuid: String,
    addon_version: String,
    addon_files: HashMap<String, AddonFile>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct AddonFile {
    file_path: String,
    file_size: u64,
    file_hash: String,
}

fn save_index(files_cache: &FilesCache) {
    let mut files_index = HashMap::new();

    for (_, local_addon) in files_cache.map.iter() {
        let addon_name = format!("{}", local_addon.name);
        let addon_uuid = format!("{}", local_addon.uuid);
        let addon_version = format!("{}", local_addon.version);
        let mut addon_files = HashMap::new();

        for (_, file_index) in local_addon.files.iter() {
            let addon_file = AddonFile {
                file_path: format!("{}", file_index.relative_filepath),
                file_size: file_index.filesize,
                file_hash: format!("{}", file_index.hash),
            };
            addon_files.insert(format!("{}", file_index.relative_filepath), addon_file);
        }

        let addon = Addon {
            addon_name: format!("{}", addon_name),
            addon_uuid,
            addon_version,
            addon_files,
        };

        files_index.insert(addon_name, addon);
    }

    let json = json!(files_index);

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

    file.write_all(json_string.as_bytes()).expect("Something went wrong while writing the index file");
}

fn is_file_ignored(dir_entry: &DirEntry, ignore_patterns: &Vec<Pattern>, settings: &Settings) -> bool {
    if dir_entry.path().is_dir() {
        return true;
    }

    if settings.ignore_hidden && dir_entry.file_name().to_str().unwrap().starts_with(".") {
        return true;
    }

    for ignore_pattern in ignore_patterns {
        if ignore_pattern.matches(dir_entry.file_name().to_str().unwrap()) {
            return true;
        }
    }

    false
}

fn get_files_in_directory(directory: &str, settings: &Settings) -> Vec<DirEntry> {
    let ignore_patterns = settings.get_ignore_patterns();

    let mut files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(directory).follow_links(settings.follow_links) {
        let dir_entry = entry.unwrap();
        if !is_file_ignored(&dir_entry, &ignore_patterns, settings) {
            files.push(dir_entry);
        }
    }

    files
}

#[derive(Serialize, Deserialize)]
struct FilesCache {
    map: HashMap<String, LocalAddon>
}

fn generate_version() -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{}", now.format("%Y%m%d-%H%M%S"))
}

impl FilesCache {
    fn remove_old_files(&mut self, files: &Vec<DirEntry>) {
        let mut files_set = HashSet::new();
        for dir_entry in files.iter() {
            files_set.insert(dir_entry.path().display().to_string());
        }
        for (_, addon_index) in self.map.iter_mut() {
            addon_index.files.retain(|x, _| files_set.contains(x.as_str()));
        }
    }

    fn create_addon_index(&mut self, addon_name: &String) -> bool {
        if !self.map.contains_key(addon_name) {
            let uuid = web_api::get_addon_uuid(&addon_name);

            self.map.insert(
                addon_name.clone(),
                LocalAddon {
                    name: addon_name.clone(),
                    uuid,
                    version: generate_version(),
                    files: HashMap::new(),
                },
            );

            return true;
        }

        false
    }

    fn get_addon_index(&mut self, addon_name: &String) -> &mut LocalAddon {
        self.map.get_mut(addon_name).unwrap()
    }
}

fn index_directory(directory: &str, index: &mut FilesCache, settings: &Settings) {
    let start = Instant::now();

    println!("Searching for files in {}", directory);
    let files = get_files_in_directory(directory, &settings);
    println!("Found {} files in {}", files.len(), directory);

    index.remove_old_files(&files);

    println!("Indexing files...");
    let mut progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.format("[=> ]");

    let mut updated_addons = HashMap::new();

    // create entries for addons if not yet in index
    for dir_entry in files {
        let absolute_filepath = dir_entry.path().display().to_string();
        let relative_filepath = extract_relative_filepath(&absolute_filepath);
        let addon_name = extract_addon_name(&relative_filepath);
        let new_entry_created = index.create_addon_index(&addon_name);

        let addon_index = index.get_addon_index(&addon_name);
        if new_entry_created {
            updated_addons.insert(format!("{}", &addon_index.uuid), format!("{}", &addon_index.version));
        }

        let metadata = fs::metadata(dir_entry.path()).unwrap();
        let filesize = metadata.len();
        let created = metadata.created().unwrap();

        let mut create_file_index = false;
        if addon_index.files.contains_key(absolute_filepath.as_str()) {
            let existing_file_index = addon_index.files.get(absolute_filepath.as_str()).unwrap();

            let existing_filesize = existing_file_index.filesize;
            let existing_created = existing_file_index.created;

            if filesize != existing_filesize || created != existing_created {
                addon_index.files.remove(absolute_filepath.as_str());
                create_file_index = true;
            }
        } else {
            create_file_index = true;
        }

        if create_file_index {
            let hash = hashing::hash_file(&dir_entry);

            let file_index = FileIndex {
                relative_filepath,
                absolute_filepath,
                created,
                filesize,
                hash,
            };

            addon_index.add_file_index(file_index);
            updated_addons.insert(format!("{}", &addon_index.uuid), format!("{}", &addon_index.version));
        }

        progress_bar.inc();
    }

    web_api::update_addons(updated_addons);

    let duration: Duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


fn main() {
    let settings = settings::load_settings();
    let map = load_filecache();
    let mut index = FilesCache { map };
    index_directory(&settings.directory, &mut index, &settings);
    save_index(&index);
    save_filecache(index);
}
