(ns clj-three.core
    (:require cljsjs.three))

(enable-console-print!)
;; Set *print-fn* to console.log

(def unit-width 90)
(def unit-height 45)

(defn init []
  (let [scene (js/THREE.Scene.)
        fog (-> scene (.-fog) (js/THREE.FogExp2. 0xcccccc 0015))
        renderer (js/THREE.WebGLRenderer.)
        container (-> js/document (.getElementById "app"))
        camera (js/THREE.PerspectiveCamera.
                60
                (/ (.-innerWidth js/window) (.-innerHeight js/window))
                1
                2000)]
    (*print-fn* fog)
    (aset camera "position" "y" 20)
    (aset camera "position" "x" 0)
    (aset camera "position" "z" 0)
    ))

(defn old-init []
  (let [scene (js/THREE.Scene.)
        p-camera (js/THREE.PerspectiveCamera. view-angle aspect near far)
        box (js/THREE.BoxGeometry. 200 200 200)
        mat (js/THREE.MeshBasicMaterial.
             (js-obj "color" 0xff0000
                     "wireframe" true))
        mesh (js/THREE.Mesh. box mat)
        renderer (js/THREE.WebGLRenderer.)]
    (aset p-camera "name" "p-camera")
    (aset p-camera "position" "z" 500)
    (aset mesh "rotation" "x" 45)
    (aset mesh "rotation" "y" 0)
    (.setSize renderer 500 500)

    (.add scene p-camera)
    (.add scene mesh)
    (.appendChild js/document.body (.-domElement renderer))

    (defn render []
      (aset mesh "rotation" "y" (+ 0.01 (.-y (.-rotation mesh))))
      (.render renderer scene p-camera))

    (defn animate []
      (.requestAnimationFrame js/window animate)
      (render))

    (animate)))

(init)
