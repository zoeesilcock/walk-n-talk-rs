# Walk and Talk

Hello world project built to learn Bevy and undestand how the ergonomics are.

![Screenshot](/screenshot.png)

## Development

### Run
```
cargo run
```

### Run with hot reload on Linux/MacOS
```
cargo watch -w systems -w components -x "build -p systems --features dynamic"
cargo run --features reload
```

### Run with hot reload on Windows
```
cargo watch -w systems -w components -x "build -p systems --features dynamic"
cargo run --features reload --target-dir "target-bin" 
```
