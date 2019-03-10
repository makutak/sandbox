(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]
            [clojure.tools.cli :refer [parse-opts]]))
(def cli-options
  [["-h" "--help"]
   ["-m" "--message"]
   ["-c" "--config"]])

(defn -main
  [& args]
  (parse-opts args cli-options))
