# Walk and Talk

Hello world project built to learn Bevy and undestand how the ergonmics are.

## Development

### Run on Linux/MacOS
```
cargo watch -w systems -w components -x "build -p systems --features dynamic"
cargo run --features reload
```

### Run on Windows
```
cargo watch -w systems -w components -x "build -p systems --features dynamic"
cargo run --features reload --target-dir "target-bin" 
```
