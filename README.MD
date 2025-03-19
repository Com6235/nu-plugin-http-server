# nu_plugin_http_server

A plugin for [nushell](https://github.com/nushell/nushell), which adds an embedded HTTP server for sharing data right from the shell!

## Commands
- `http serve` - Serve piped content on a web server

## Building

Use `toolkit.nu` to download and patch the dependecies.
Then do 
```shell
cargo build --release
```
