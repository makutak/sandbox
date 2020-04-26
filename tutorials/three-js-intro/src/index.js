import * as THREE from "three";

window.addEventListener("DOMContentLoaded", init);

const WIDTH = 960;
const HEIGHT = 540;

function init() {
  const renderer = new THREE.WebGLRenderer({
    canvas: document.querySelector("#my-canvas"),
  });
  renderer.setSize(WIDTH, HEIGHT);
  renderer.pixelRatio(window.devicePixelRatio);
}
