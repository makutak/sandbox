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

  // new THREE.PerspectiveCamera(画角, アスペクト比, 描画開始距離, 描画終了距離)
  const camera = new THREE.PerspectiveCamera(45, 800 / 600, 1, 10000);
  console.log("camera: ", camera);


  /**
   * 立方体は「メッシュ」という表示オブジェクトを使用して作成する。
   * メッシュを作るにはジオメトリ(形状)とマテリアル(素材)を用意する必要がある。
   */

  // new THREE.BoxGeometry(幅, 高さ, 奥行き)
  // - ジオメトリは頂点情報や面情報を持ってる。
  const geometry = new THREE.BoxGeometry(500, 500, 500);

  // - マテリアルは色や質感の情報を持ってる。
  const material = new THREE.MeshStandardMaterial({ color: 0x0000ff });

  // new THREE.Mesh(ジオメトリ,マテリアル)
  const mesh = new THREE.Mesh(geometry, material);
  // シーンに追加
  scene.add(mesh);
}
