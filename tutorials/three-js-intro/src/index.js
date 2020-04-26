import * as THREE from "three";

window.addEventListener("DOMContentLoaded", init);

function init() {
  const renderer = new THREE.WebGLRenderer({
    canvas: document.querySelector("#my-canvas"),
  });
  console.log("renderer: ", renderer);
}
