(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]))

(defn -main
  [& args]
  (cond
    (= (first args) "--help") (prn "help")
    (= (first args) "--config") (prn "config")
    (= (first args) "--message") (prn "message")
    :else (prn "see help")))
