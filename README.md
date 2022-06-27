<h1 align="center">
  <img src="assets/logo256.png" width=200 height=200/><br>
  Check Buddy
  </a>
</h1>

<p>A standalone Chess engine written in Rust, using <a href="https://github.com/bevyengine/bevy/" target="_blank">Bevy</a> for GUI
</p>

![](assets/footage.gif)

# Running

### Debug (without Bevy)

```commandline
cargo run 
```

### Debug (with Bevy)

```commandline
cargo run --release
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
