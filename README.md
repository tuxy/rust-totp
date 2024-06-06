# rust-totp

Very basic CLI TOTP application written in rust (Also my first rust app). Initial Features include:
 - Multiple TOTP codes, searched by argument
 - Cross-platform config file (using platform_dirs)
 - Clipboard functionality (Copy code & Clear code on exit)
 - TOTP Timer (30s only)

Planned features:
 - Custom timer for TOTP
 - Custom configuration file
 - Secrets encryption and authentication