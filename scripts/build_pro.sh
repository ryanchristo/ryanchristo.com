cargo build --release

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/release/ryanchristo_client.wasm --no-typescript
