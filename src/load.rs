use toml::from_str;
use std::{ time::{ SystemTime, UNIX_EPOCH }, fs, collections::HashMap };
use serde::Deserialize;
use platform_dirs::AppDirs;

#[derive(Debug, Deserialize)]
pub struct Item {
    name: String,
    secret: String,
    time: u64,
} 

pub fn time_left(time: u64) -> u64 {
    let start = SystemTime::now();
    let time_since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards???? Check system clock.")
        .as_secs(); 
        time - time_since_epoch % time
}

pub fn string_vec(data: &str) -> Vec<String> {
    let toml_file = &load_keys()["key"];
    let mut return_data: Vec<String> = vec![];

    for i in toml_file {
        if data == "name" {
            return_data.push(i.name.clone());
        } else {
            return_data.push(i.secret.clone());
        }
    }
    return_data
}

pub fn unsigned_vec() -> Vec<u64> {
    let toml_file = &load_keys()["key"];
    let mut return_data: Vec<u64> = vec![];

    for i in toml_file {
        return_data.push(i.time);
    }

    return_data
}

pub fn load_keys() -> HashMap<String, Vec<Item>> {
    
    let app_dirs = AppDirs::new(Some("rstotp"), true).unwrap();
    let config_file_path = app_dirs.config_dir.join("secrets.toml");

    fs::create_dir_all(&app_dirs.config_dir).expect("Could not create config directory.");

    // TODO: Use a more reliable method to load TOML file directly into a TOML, without converting to string and back. Any minor error in TOML or string would cause program to flip out
    let items_string: String = fs::read_to_string(config_file_path.into_os_string())
        .expect("Could not load toml. Check format?");
    let items_table: HashMap<String, Vec<Item>> = from_str(&items_string)
        .expect("Could not load TOML Secrets. Please check formatting");
    
    items_table
}