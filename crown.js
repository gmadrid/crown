import init, { run_crown_app } from './pkg/crown.js';
async function main() {
   await init('/pkg/crown_bg.wasm');
   run_crown_app();
}
main()
