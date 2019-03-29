(ns leaflet-tutorial.prod
  (:require
    [leaflet-tutorial.core :as core]))

;;ignore println statements in prod
(set! *print-fn* (fn [& _]))

(core/init!)
