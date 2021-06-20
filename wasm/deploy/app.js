// Inject the file loading button at runtime
document.getElementById("load").innerHTML = '<input type="file" file=null id="file-selector" accept=".step,.stp">';
import * as SCENE from './scene.js';

const worker = new Worker("worker.js");
const setStatus = function(s) {
    document.getElementById("status").innerHTML = s;
    loadTimeShowing = false;
};
let startTime, loadTimeShowing = false;
SCENE.addCameraCallback(function () {
    if (loadTimeShowing) {
        setStatus("");
    }
});

worker.onmessage = function(e) {
    setStatus("Building scene...");
    SCENE.loadMesh(e.data);
    fileSelector.disabled = false;
    exampleSelector.disabled = false;
    const d = new Date();
    const now = d.getTime();
    const dt_sec = (now - startTime) / 1000.0;
    setStatus("Loaded in " + dt_sec.toPrecision(3) + " sec");
    loadTimeShowing = true;
    if (targetAxis) {
        if (targetAxis == "X") {
            SCENE.axisX();
        } else if (targetAxis == "Y") {
            SCENE.axisY();
        } else if (targetAxis == "Z") {
            SCENE.axisZ();
        }
        targetAxis = null;
    }
}
const loadMeshFromString = function(s) {
    const d = new Date();
    startTime = d.getTime();
    setStatus("Parsing & triangulating...");
    worker.postMessage(s);
}

const fileSelector = document.getElementById('file-selector');
const exampleSelector = document.getElementById('example-selector');
fileSelector.addEventListener('change', (event) => {
    fileSelector.disabled = true;
    exampleSelector.disabled = true;
    exampleSelector.selectedIndex = 0;

    // Hide the "model source" text by the examples list
    let src = document.getElementById("example_src");
    src.style.visibility = "hidden";

    const fileList = event.target.files;
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.addEventListener('load', (event) => {
        loadMeshFromString(event.target.result);
    });
    setStatus("Uploading...");
    reader.readAsText(file);
});

setStatus("");

document.getElementById("axis_label");
const setAxisLabel = function(s) {
    axis_label.innerHTML = s;
    axis_label.style.visibility = s ? "visible" : "hidden";
};
document.getElementById("axisX").onclick = SCENE.axisX;
document.getElementById("axisY").onclick = SCENE.axisY;
document.getElementById("axisZ").onclick = SCENE.axisZ;

document.getElementById("axisX").onmouseover = function() { setAxisLabel("X"); };
document.getElementById("axisY").onmouseover = function() { setAxisLabel("Y"); };
document.getElementById("axisZ").onmouseover = function() { setAxisLabel("Z"); };
document.getElementById("axisX").onmouseout = function() { setAxisLabel(""); };
document.getElementById("axisY").onmouseout = function() { setAxisLabel(""); };
document.getElementById("axisZ").onmouseout = function() { setAxisLabel(""); };

let exampleList;
function populateExamples(d) {
    var i = 1;
    exampleList = [];
    for (const ex of d) {
        var opt = document.createElement('option');
        opt.text = ex[0];
        opt.value = i;
        i += 1;
        exampleSelector.add(opt);
        exampleList.push(ex);
    }
}
let targetAxis;
exampleSelector.onchange = function(e) {
    const v = e.target.options[e.target.selectedIndex].value;
    console.log(v);
    if (v >= 1) {
        setStatus("Downloading...");

        // Show the "Model Source" text
        const src = document.getElementById("example_src");
        src.style.visibility = "visible";

        const ex = exampleList[v - 1];
        src.href = ex[2];

        fileSelector.disabled = true;
        exampleSelector.disabled = true;
        if (ex.length == 4) {
            targetAxis = ex[3];
        }

        fetch(ex[1])
            .then(response => response.text())
            .then(text => loadMeshFromString(text))
    }
}
fetch('examples.json')
    .then(response => response.json())
    .then(data => populateExamples(data));
