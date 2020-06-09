(ns clj-three.core
    (:require cljsjs.three))

(enable-console-print!)
;; Set *print-fn* to console.log

(def unit-width 90)
(def unit-height 45)

(defn init
  []
  (let [scene (js/THREE.Scene.)
        ;;fog (.-fog scene (js/THREE.FogExp2. 0xcccccc 0015))
        renderer (js/THREE.WebGLRenderer.)
        container (-> js/document (.getElementById "app"))
        camera (js/THREE.PerspectiveCamera.
                60
                (/ (. js/window -innerWidth ) (. js/window -innerHeight))
                1
                2000)]
    (aset scene "fog" (js/THREE.FogExp2. 0xcccccc 0015))
    ;; renderer
    (.setClearColor renderer (.. scene -fog -color))
    (.setPixelRatio renderer (. js/window -devicePixelRatio))
    (.setSize renderer (. js/window -innerWidth ) (. js/window -innerHeight))
    ;; container
    (.appendChild container (. renderer -domElement))
    ;; camera
    (aset camera "position" "y" 20)
    (aset camera "position" "x" 0)
    (aset camera "position" "z" 0)
    (.add scene camera)
    (. js/window addEventListener "resize" on-window-resize false)))

(defn create-maze-cubes
  [])

(defn add-lights
  [])

(defn on-window-resize
  []
  (prn "on-window-resize!!"))

(defn animate
  [])

(defn render
  [])

;; (defn old-init []
;;   (let [scene (js/THREE.Scene.)
;;         p-camera (js/THREE.PerspectiveCamera. view-angle aspect near far)
;;         box (js/THREE.BoxGeometry. 200 200 200)
;;         mat (js/THREE.MeshBasicMaterial.
;;              (js-obj "color" 0xff0000
;;                      "wireframe" true))
;;         mesh (js/THREE.Mesh. box mat)
;;         renderer (js/THREE.WebGLRenderer.)]
;;     (aset p-camera "name" "p-camera")
;;     (aset p-camera "position" "z" 500)
;;     (aset mesh "rotation" "x" 45)
;;     (aset mesh "rotation" "y" 0)
;;     (.setSize renderer 500 500)

;;     (.add scene p-camera)
;;     (.add scene mesh)
;;     (.appendChild js/document.body (.-domElement renderer))

;;     (defn render []
;;       (aset mesh "rotation" "y" (+ 0.01 (.-y (.-rotation mesh))))
;;       (.render renderer scene p-camera))

;;     (defn animate []
;;       (.requestAnimationFrame js/window animate)
;;       (render))

;;     (animate)))

(init)
