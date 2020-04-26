import * as THREE from "three";

window.addEventListener("DOMContentLoaded", init);

const WIDTH = 960;
const HEIGHT = 540;

function init() {
  const renderer = new THREE.WebGLRenderer({
    canvas: document.querySelector("#my-canvas"),
  });
  renderer.setSize(WIDTH, HEIGHT);
  renderer.setPixelRatio(window.devicePixelRatio);

  const scene = new THREE.Scene();
  console.log("scene: ", scene);

  const camera = new THREE.PerspectiveCamera(45, 800 / 600, 1, 10000);
  console.log("camera: ", camera);
}
