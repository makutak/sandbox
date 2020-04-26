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

  // new THREE.PerspectiveCamera(画角, アスペクト比, 描画開始距離, 描画終了距離)
  const camera = new THREE.PerspectiveCamera(45, 800 / 600, 1, 10000);
  camera.position.set(0, 0, +1000);

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

  // new THREE.DirectionalLight(色)
  const light = new THREE.DirectionalLight(0xffffff);
  // 光の強さを倍に
  light.intensity = 2;
  // ライトの位置を変更
  light.position.set(1, 1, 1);
  // シーンに追加
  scene.add(light);

  renderer.render(scene, camera);
}
