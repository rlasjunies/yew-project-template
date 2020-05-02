
wasm-pack build --debug --no-typescript --out-name spa --target web

<!-- wasm-pack build --debug --no-typescript -d ./target/pkg/debug --out-name spa --target web -->


<!-- wasm-gc target/wasm32-unknown-unknown/debug/wasm_sample.wasm -->
wasm-gc pkg/spa_bg.wasm