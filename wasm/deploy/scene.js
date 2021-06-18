import * as THREE from 'https://cdn.skypack.dev/three@0.129.0'
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.129.0/examples/jsm/controls/OrbitControls.js';

let camera, controls, scene, renderer;

init();
render();

export function init() {

    scene = new THREE.Scene();
    scene.background = new THREE.Color(0xcccccc);

    const canvas = document.getElementById("canvas");
    renderer = new THREE.WebGLRenderer({ antialias: true, canvas: canvas });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(canvas.width / window.devicePixelRatio, canvas.height / window.devicePixelRatio);

    // build a camera with lights attached
    camera = new THREE.PerspectiveCamera(60, canvas.width / canvas.height, 1, 1000);
    camera.position.set(400, 200, 0);
    camera.add(new THREE.DirectionalLight());
    camera.add(new THREE.HemisphereLight());
    scene.add(camera);

    // controls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.listenToKeyEvents(window); // optional
    controls.addEventListener('change', render);

    controls.screenSpacePanning = true;
    controls.minDistance = 100;
    controls.maxDistance = 500;

    window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
    const canvas = document.getElementById("canvas");
    renderer.setPixelRatio(window.devicePixelRatio);
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

    const material = new THREE.MeshPhongMaterial({vertexColors: true, side: THREE.DoubleSide});
    //const material = new THREE.MeshNormalMaterial({ side: THREE.DoubleSide });
    const mesh = new THREE.Mesh(geometry, material);
    mesh.updateMatrix();
    mesh.matrixAutoUpdate = false;
    mesh.name = "step";

    scene.add(mesh);
    render();
}
