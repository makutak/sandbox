(ns leaflet-tutorial.core
  (:require [reagent.core :as r]
            [react-leaflet :as leaflet]))


(def Map (r/adapt-react-class leaflet/Map))
(def TileLayer (r/adapt-react-class leaflet/TileLayer))
(def Marker (r/adapt-react-class leaflet/Marker))
(def Popup (r/adapt-react-class leaflet/Popup))

(def position [51.505 -0.09])


(defn leaflet-map
  []
  [Map {:center position :zoom 13}
   [TileLayer {:url "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
               :attribution "&copy; <a href=&quot;http://osm.org/copyright&quot;>OpenStreetMap</a> contributors"}]
   [Marker {:position position}
    [Popup "A pretty CSS3 popup.<br />Easily customizable." ]]])

;; -------------------------
;; Views


(defn home-page
  []
  [leaflet-map])

;; -------------------------
;; Initialize app

(defn mount-root
  []
  (r/render [home-page] (.getElementById js/document "app")))

(defn init!
  []
  (mount-root))
