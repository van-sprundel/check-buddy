<h1 align="center">
  <img src="crates/check-buddy-bevy/assets/logo256.png" width=200 height=200/><br>
  ♟️ Check Buddy
  </a>
</h1>

<p>A Chess engine written in Rust, using <a href="https://github.com/bevyengine/bevy/" target="_blank">🐦Bevy</a> as GUI
</p>

![](crates/check-buddy-bevy/assets/footage.gif)

Currently, the Bevy implementation is flawed. Pieces do not update correctly. It'd be better if pieces updated with every move.

If you feel like implementing something like this, (Whether that's in Bevy, Macroquad, WGPU, etc.) feel free!

# ⚙️Running

### Debug (without Bevy)

```commandline
cargo run 
```

### Debug (with Bevy)

```commandline
cargo run --release
```

# 🧱WASM

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
```
Windows
```
wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\check-buddy.wasm
```
Unix/macOS
```
wasm-bindgen --out-dir ./out/ --target web target/wasm32-unknown-unknown/release/check-buddy.wasm
```
