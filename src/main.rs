use rust_otp;
use serde::Deserialize;
use toml::from_str;
use platform_dirs::AppDirs;
use indicatif::ProgressBar;
use ctrlc;
use arboard::Clipboard;

use clap::Parser;

use std:: {
    collections::HashMap, fs, io, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::{Duration, SystemTime, UNIX_EPOCH}
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String
}

#[derive(Deserialize, Debug)]
struct Settings {
    enable_clipboard: bool,
}

// Item struct to deserialize TOML
#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    secret: String,
    time: u64,
}

fn main() -> io::Result<()> {
    // Setting termination (Ctrl-C Handler)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Parse arguments & Initialise clipboard
    let args = Args::parse();
    let mut clipboard = Clipboard::new().expect("Could not initialise clipboard.");

    // Setting Ctrl-C handler to "gracefully" handle error
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Cannot set Ctrl-C handler.");

    // Load keys (name & secret) from HashMap
    let keys = &load_keys()["key"];

    // Load settings into settings
    let settings = load_settings();

    let _test = load_settings();
    // Decodes secrets and prints out code + time left
    // Variable to (very shoddily) check if code has been found 
    let mut any: bool = false;
    for i in keys {
        if args.name.eq(&i.name) {
                            
            // Create a new indicatif progress bar
            let pb = ProgressBar::new(i.time);

            while running.load(Ordering::SeqCst) { // Ctrl-C handler
                // Loop for timer & clipboard
                // Finds TOTP code
                let code = match rust_otp::make_totp(&i.secret.to_ascii_uppercase(), i.time, 0) {
                    Ok(u32) => u32,
                    Err(_otperror) => {
                        panic!("Failed to generate codes from secret. Check if codes are valid.");
                    },
                };
                // Checks if clipboard is enabled
                if settings.enable_clipboard {
                    clipboard.set_text(code.to_string()).unwrap();
                }
                // Sleep to prevent timer from moving too quickly
                // TODO: Perfectly matches presented time for TOTP > Sometimes moves too quickly
                thread::sleep(Duration::from_millis(250));
                pb.set_position(time_left(i.time));
                any = true;
            }
        }
    }

    // Checks if keys exist. If they don't, then warn the user
    if any {
        clipboard.clear().expect("Unable to clear clipboard"); // Clear clipboard, exit and return ok
        println!("Exiting & Clearing clipboard...");
        Ok(())
    } else {
        println!("WARN: Found no keys.");
        Ok(())
    }
}

// Function that returns time left of a TOTP (30 seconds ONLY)
fn time_left(time: u64) -> u64 {
    let start = SystemTime::now();
    let time_since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards???? Check system clock.")
        .as_secs();
    time - time_since_epoch % time 
}

// Loads keys from a TOML file into a HashMap
fn load_keys() -> HashMap<String, Vec<Item>> {
    
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

// Loads settings into struct Settings. May be a more idiomatic way to do this without a seperate function.
fn load_settings() -> Settings {
    
    let app_dirs = AppDirs::new(Some("rstotp"), true).unwrap();
    let config_file_path = app_dirs.config_dir.join("settings.toml");

    fs::create_dir_all(&app_dirs.config_dir).expect("Could not create config directory.");

    let items_string = fs::read_to_string(config_file_path.into_os_string())
        .expect("Could not load settings.toml. Check if file exists.");
    let items_table: Settings = from_str(&items_string)
        .expect("Could not load settings.toml. Check format?");

    items_table
}