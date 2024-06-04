use rust_otp;
use serde::Deserialize;
use toml::from_str;
use platform_dirs::AppDirs;
use indicatif::ProgressBar;
use ctrlc;
use arboard::Clipboard;

use clap::Parser;

use std:: {
    collections::HashMap, fs::{self, File}, io, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self}, time::{Duration, SystemTime, UNIX_EPOCH}
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String
}


#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    secret: String,
}

fn main() -> io::Result<()> {
    // Setting termination (Ctrl-C Handler)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Parse arguments & Initialise clipboard
    let args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();


    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Create a new indicatif progress bar
    let pb = ProgressBar::new(30);

    let keys = &load_keys()["key"];
    // Decodes secrets and prints out code + time left
    for i in keys {
        if args.name.eq(&i.name) {
            while running.load(Ordering::SeqCst) {
                // Loop for timer & clipboard
                // Finds TOTP code
                let code = match rust_otp::make_totp(&i.secret.to_ascii_uppercase(), 30, 0) {
                    Ok(u32) => u32,
                    Err(_otperror) => panic!("Failed to generate codes from secret. Check if codes are valid."),
                };
                // Sets Clipboard to code
                clipboard.set_text(code.to_string()).unwrap();
        
                thread::sleep(Duration::from_millis(500));
                pb.set_position(time_left());
            }
        }
    }

    // Clear clipboard, exit and return ok
    clipboard.clear().expect("Unable to clear clipboard");
    println!("Exiting & Clearing clipboard...");
    Ok(())
}

// Function that returns time left of a TOTP (30 seconds ONLY)
fn time_left() -> u64 {
    let start = SystemTime::now();
    let time_since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards???? Check system clock.")
        .as_secs();
    30 - time_since_epoch % 30
}

// Loads keys from a TOML file into a HashMap
fn load_keys() -> HashMap<String, Vec<Item>> {
    
    let app_dirs = AppDirs::new(Some("rstotp"), true).unwrap();
    let config_file_path = app_dirs.config_dir.join("secrets.toml");

    fs::create_dir_all(&app_dirs.config_dir).unwrap();

    let _file = if config_file_path.exists() {
        File::open(config_file_path.clone()).unwrap()
    } else {
        File::create(config_file_path.clone()).unwrap()
    };

    let items_string: String = fs::read_to_string(config_file_path.into_os_string())
        .expect("Could not load toml. Check format?");
    let items_table: HashMap<String, Vec<Item>> = from_str(&items_string)
        .unwrap();
    
    items_table
}