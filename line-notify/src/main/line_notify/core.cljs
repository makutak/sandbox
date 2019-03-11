(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]
            [clojure.tools.cli :refer [parse-opts]]
            [clojure.string :as s]))

(def cli-options
  [["-h" "--help" "Show help"]
   ["-m" "--message" "You want to send message"]
   ["-c" "--config" "Your LINE API Token"]])


(defn usage
  [options-summary]
  (s/join \newline
          ["This is LIne Notify."
           ""
           "Usage: Line Notify [options] action."
           "Options:"
           options-summary
           ""]))

(defn notify
  [token message]
  (println token message))

(defn -main
  [& args]
  (let [{:keys [options summary arguments]} (parse-opts args cli-options)]
    (println  (parse-opts args cli-options))
    (println arguments)
    (cond
      (:help options) (println (usage summary))
      (and (:config options) (:message options)) (notify (:config options) (:message options))
      :else (println "this is line-notify!"))))
