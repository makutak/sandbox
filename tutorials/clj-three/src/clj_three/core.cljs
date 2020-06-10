(ns clj-three.core
    (:require cljsjs.three))

(enable-console-print!)
;; Set *print-fn* to console.log

(def unit-width 90)
(def unit-height 45)
(def scene)
(def camera)
(def renderer)

(declare create-maze-cubes add-lights on-window-resize animate render degrees->radians)

(defn init
  []
  (let [container (-> js/document (.getElementById "app"))]
    (set! scene (js/THREE.Scene.))
    (set! renderer (js/THREE.WebGLRenderer.))
    (set! camera (js/THREE.PerspectiveCamera.
                  60
                  (/ (. js/window -innerWidth ) (. js/window -innerHeight))
                  1
                  2000))
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
    (create-maze-cubes)
    (. js/window addEventListener "resize" on-window-resize false)))

(defn create-maze-cubes
  []
  (let [cubu-geo (js/THREE.BoxGeometry. unit-width unit-height unit-width)
        cube-met (js/THREE.MeshPhongMaterial. (js-obj {:color 0x81cfe0}))
        cube (js/THREE.Mesh. cubu-geo cube-met)]
    (.add scene cube)
    (*print-fn* cube)
    (aset cube "position" "y" (/ unit-height 2))
    (aset cube "position" "x" 0)
    (aset cube "position" "z" -100)
    (aset cube "position" "rotation" "y" )
    (degrees->radians 30)))

(defn add-lights
  [])

(defn on-window-resize
  []
  (prn "on-window-resize!!"))

(defn animate
  [])

(defn render
  [])

(defn degrees->radians
  [degrees]
  (/ (* degrees js/Math.PI) 180))

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
