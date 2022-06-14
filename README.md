
![](assets/logo128.png)

# Check Buddy

A standalone Chess engine written in Rust, using [Bevy](https://github.com/bevyengine/bevy/) for GUI


![](assets/footage.gif)

# Running

### Debug

```commandline
cargo run
```

# WASM

## Serving

Make sure you have WASM support and `wasm-server-runner` installed:

```commandline
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```

Run with:

```commandline
cargo run --target wasm32-unknown-unknown
```

## Building

Make sure you have wasm-bindgen installed:

```commandline
cargo install -f wasm-bindgen-cli
```

Run with:

```commandline
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\chess.wasm
```
