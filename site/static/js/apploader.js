import init, { run_app } from '/spa/spa.js';
async function main() {
  await init('/spa/spa_bg.wasm');
  run_app();
}
main()
