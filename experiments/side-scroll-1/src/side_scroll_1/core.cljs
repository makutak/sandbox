(ns side-scroll-1.core
    (:require ))

(enable-console-print!)

(def canvas (.getElementById js/document "app"))
(def ctx (.getContext canvas "2d"))

(def ball-radius 10)
(def x ball-radius)
;;(def y (- (. canvas -height) ball-radius))
(def y ball-radius)


(def t 0)
(def dt 0.1)
(def v 10)
(def g 9.8)
(def dx 2)
(def dy -2)

(defn draw-ball []
  (.beginPath ctx)
  (.arc ctx x y ball-radius 0 (* Math.PI 2) false)
  (aset ctx "fillStyle" "#0095DD")
  (.fill ctx)
  (.closePath ctx))

(defn draw []
  (.clearRect ctx 0 0 (. canvas -width) (. canvas -height))
  (draw-ball)
  ;; 水平投射
  (set! t (+ t dt))
  (set! x (+ x (* v t)))
  (set! y (+ y (* (/ 1 2) g (* t t))))
  (js/requestAnimationFrame draw)
  )
(draw)
