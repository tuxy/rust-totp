# rust-totp

[demo]()

## Features
Very basic CLI TOTP application written in rust (Also my first rust app). Initial Features include:
 - Multiple TOTP codes, searched by argument
 - Cross-platform config file (using platform_dirs)
 - Clipboard functionality (Copy code & Clear code on exit)
 - TOTP Timer (Any time)
 - Custom configuration file

Planned features:
 - TODO Gui (Planning to use egui for ease and performance)
 - Better error handling
 - Config check
 - Secrets encryption and authentication

## Build for desktop

```cargo build --release```

## Build for web

Refer to [the web deploy](https://github.com/emilk/eframe_template?tab=readme-ov-file#web-deploy) section for egui, but essentially:

Install trunk

```cargo install --locked trunk```

Build for web to get ```dist/``` directory

```trunk build release```