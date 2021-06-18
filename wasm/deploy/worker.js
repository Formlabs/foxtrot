importScripts("wasm.js");

const { foxtrot, init_log } = wasm_bindgen;
async function run() {
    await wasm_bindgen();
    init_log();

    onmessage = function(e) {
        var triangles = foxtrot(e.data);
        postMessage(triangles);
    }
}
run();
