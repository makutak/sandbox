(ns leaflet-tutorial.core
  (:require [reagent.core :as r]
            [react-leaflet :refer [Map TileLayer Marker Popup]]))

(def position [35.689236 139.735744])

;; -------------------------
;; Views
(defn leaflet-map
  []
  [:> Map {:center position :zoom 16}
   [:> TileLayer {:url "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                    :attribution "&copy; <a href=&quot;http://osm.org/copyright&quot;>OpenStreetMap</a> contributors"}]
   [:> Marker {:position position}
    [:> Popup "A pretty CSS3 popup.<br />Easily customizable." ]]])



;; -------------------------
;; Initialize app

(defn mount-root
  []
  (r/render [leaflet-map] (.getElementById js/document "app")))

(defn init!
  []
  (mount-root))
