importScripts("wasm.js");

const { step_to_triangle_buf, init_log } = wasm_bindgen;
async function run() {
    await wasm_bindgen();
    init_log();

    onmessage = function(e) {
        var triangles = step_to_triangle_buf(e.data);
        postMessage(triangles);
    }
}
run();
