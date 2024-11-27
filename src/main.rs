#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on windows

use eframe::egui::{ self, Align2, FontId, RichText, Separator };
use std::{ cmp::Ordering, thread, time::Duration };
use rust_otp;
use arboard::Clipboard;
use egui_toast::*;

mod load; // Moved seperate functions to load.rs

// UI Constants (For quick adjustments...)
const ALIGNMENT: usize = 6;
const ALIGNMENT_TIME: usize = 2;
const ALIGNMENT_STRING: usize = 12;
const DELAY: Duration = Duration::from_millis(5);

fn main() -> Result<(), eframe::Error> {

    // Adding a config system to be able to switch settings such as always on top, etc...
    env_logger::init(); 
    let options = eframe::NativeOptions { // Setting native window options. TODO: Options to change this?
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 640.0])
            .with_always_on_top(),
            // Always on top <-----
        ..Default::default()
    };
    eframe::run_native(
        "rust-totp", 
        options, // Option presets?
        Box::new(|_cc| {
            Ok(Box::<Secrets>::default())
        }),
    )

}
struct Secrets { // Variable init for vars "thrown" into the impl for App
    names: Vec<String>,
    keys: Vec<String>,
    time: Vec<u64>,
    clipboard: Clipboard,
    clipboard_timer: u16,
    toasts: Toasts,
}

impl Default for Secrets { // The vars actually "thrown" into the impl for App by default (On startup)
    fn default() -> Self {
        
        Self {
            names: load::string_vec("name"),
            keys: load::string_vec("secret"), // Functions from the load file to open CONFIG_DIR/secrets.toml
            time: load::unsigned_vec(),
            clipboard: Clipboard::new().expect("Could not initialise clipboard."),
            clipboard_timer: 0,
            toasts: Toasts::new()
                .anchor(Align2::CENTER_BOTTOM, (0.0, -10.0))
                .direction(egui::Direction::BottomUp),
        }
    }
}

impl eframe::App for Secrets {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {   
                // Iterates over entries for totp codes and lists them
                // TODO: Seperate into another file for organisation         
                for (i, _el) in self.names.iter().enumerate() {

                    let time = load::time_left(self.time[i]); // Set TOTP Time

                    let code = match rust_otp::make_totp(&self.keys[i], self.time[i], 0) {
                        Ok(u32) => u32,
                        Err(_otperror) => { // I have no idea how match-error is handled in rust... 
                            panic!("Failed to generate codes from secret. Check if codes are valid.");
                        },
                    };

                    // Entire section that shows name, time left, copy button and code
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {

                            ui.label(format!("▶ {:alignment$}", self.names[i], alignment = ALIGNMENT_STRING)); // Name of app

                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{:0alignment$}", time, alignment = ALIGNMENT_TIME)).font(FontId::proportional(20.0))); // Time label
                                if ui.button("copy").clicked() {
                                    self.clipboard.set_text(code.to_string()).expect("Could not set clipboard");
                                    self.clipboard_timer = 2000;
                                    self.toasts.add(Toast {
                                        text: "copied!".into(),
                                        kind: ToastKind::Info,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(10.0)
                                            .show_progress(true),
                                        ..Default::default()
                                    });
                                }
                            });

                        });
                        ui.add_space(140.0);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.label(RichText::new(format!("{:0alignment$}", code, alignment = ALIGNMENT)).font(FontId::proportional(40.0))); // TOTP Code
                        });
                    });
                    ui.add(Separator::default());
                }
            });
        });
        match self.clipboard_timer.cmp(&1) {
            Ordering::Greater => {self.clipboard_timer -= 1},
            Ordering::Equal => {
                self.clipboard_timer -= 1;
                self.clipboard.clear().expect("Could not clear clipboard");
            },
            Ordering::Less => {},
        }
        thread::sleep(DELAY); // Attempt to reduce CPU usage
        ctx.request_repaint(); // "Refresh" frame every 5 ms to avoid freezing on lost focus (MORE CPU USAGE)

        self.toasts.show(ctx); // Show toasts
    }
}