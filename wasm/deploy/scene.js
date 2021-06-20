import * as THREE from 'https://cdn.skypack.dev/three@0.129.0'
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.129.0/examples/jsm/controls/OrbitControls.js';

let camera, controls, scene, renderer;

init();

export function init() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0xffffff);

    const canvas = document.getElementById("canvas");
    console.log(canvas.width, canvas.height);
    renderer = new THREE.WebGLRenderer({ antialias: true, canvas: canvas });

    // build a camera with lights attached
    camera = new THREE.PerspectiveCamera(60, 1 / 0.6, 1, 1000);
    camera.position.set(200, 200, 0);
    camera.lookAt(new THREE.Vector3(0,0,0));
    camera.add(new THREE.DirectionalLight(0xffffff, 0.2));
    camera.add(new THREE.HemisphereLight(0xffffff, 0.5));
    scene.add(camera);

    onWindowResize();

    // controls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.addEventListener('change', render);

    controls.screenSpacePanning = true;
    controls.minDistance = 100;
    controls.maxDistance = 500;

    window.addEventListener('resize', onWindowResize);
}

export function addCameraCallback(f) {
    controls.addEventListener('change', f);
}

function onWindowResize() {
    const body = document.getElementById("main");
    const width = body.offsetWidth;

    const canvas = document.getElementById("canvas");
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(width, width * 0.6);
    render()
}

function render() {
    renderer.render(scene, camera);
}

export function loadMesh(buf) {
    const obj = scene.getObjectByName("step");
    if (obj) {
        scene.remove(obj);
    }

    const interleaved = new THREE.InterleavedBuffer(buf, 9);
    const geometry = new THREE.BufferGeometry();

    geometry.setAttribute('position', new THREE.InterleavedBufferAttribute(interleaved, 3, 0));
    geometry.setAttribute('normal', new THREE.InterleavedBufferAttribute(interleaved, 3, 3));
    geometry.setAttribute('color', new THREE.InterleavedBufferAttribute(interleaved, 3, 6));

    const material = new THREE.MeshPhongMaterial({vertexColors: true});
    //const material = new THREE.MeshNormalMaterial({ side: THREE.DoubleSide });
    const mesh = new THREE.Mesh(geometry, material);
    mesh.updateMatrix();
    mesh.matrixAutoUpdate = false;
    mesh.name = "step";

    scene.add(mesh);

    camera.position.set(400, 200, 0);
    camera.lookAt(new THREE.Vector3(0,0,0));
    axisX();

    render();
}

export function axisX() {
    const obj = scene.getObjectByName("step");
    if (obj) {
        const m = new THREE.Matrix4();
        m.set(1, 0, 0, 0,
              0, 1, 0, 0,
              0, 0, 1, 0,
              0, 0, 0, 1);
        obj.matrix = m;
        render();
    }
}

export function axisY() {
    const obj = scene.getObjectByName("step");
    if (obj) {
        const m = new THREE.Matrix4();
        m.set(0, 1, 0, 0,
              0, 0, 1, 0,
              1, 0, 0, 0,
              0, 0, 0, 1);
        obj.matrix = m;
        render();
    }
}
export function axisZ() {
    const obj = scene.getObjectByName("step");
    if (obj) {
        const m = new THREE.Matrix4();
        m.set(0, 0, 1, 0,
              1, 0, 0, 0,
              0, 1, 0, 0,
              0, 0, 0, 1);
        obj.matrix = m;
        render();
    }
}
