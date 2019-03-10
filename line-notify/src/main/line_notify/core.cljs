(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]
            [clojure.tools.cli :refer [parse-opts]]
            [clojure.string :as s]))

(def cli-options
  [["-h" "--help"]
   ["-m" "--message"]
   ["-c" "--config"]])


(defn usage
  [options-summary]
  (s/join \newline
          ["This is LIne Notify."
           ""
           "Usage: Line Notify [options] action."
           "Options:"
           options-summary
           ""]))

(defn -main
  [& args]
  (let [{:keys [options summary]} (parse-opts args cli-options)]
    (cond
      (:help options) (println (usage summary))
      :else (println "this is line-notify!"))))
