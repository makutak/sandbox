(ns ^:figwheel-no-load leaflet-tutorial.dev
  (:require
    [leaflet-tutorial.core :as core]
    [devtools.core :as devtools]))


(enable-console-print!)

(devtools/install!)

(core/init!)
