extern crate data_encoding;
extern crate pbr;


use std::time::{Duration, Instant};

use glob::Pattern;
use pbr::ProgressBar;
use walkdir::{DirEntry, WalkDir};

mod hashing;
mod settings;

use settings::Settings;

fn extract_relative_filepath(dir_entry: &DirEntry) -> String {
    let dir_entry_str = dir_entry.path().display().to_string();
    let index = dir_entry_str.find("@").unwrap_or(0);
    let (_, relative_filepath) = dir_entry_str.split_at(index);

    String::from(relative_filepath)
}

fn index_directory(directory: &str, settings: &Settings) {
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
    let mut hashes = Vec::new();
    let mut pb = ProgressBar::new(files.len() as u64);
    pb.format("[=> ]");
    for dir_entry in files {
        let relative_filepath = extract_relative_filepath(&dir_entry);
        println!("{}", relative_filepath);
        let hash = hashing::hash_file(&dir_entry);
        hashes.push(hash);
        pb.inc();
    }

    let duration: Duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


fn main() {
    let settings = settings::load_settings();

    for directory in &settings.observed_directories {
        index_directory(directory, &settings);
    }
}
