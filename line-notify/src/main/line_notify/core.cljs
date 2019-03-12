(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]
            [clojure.tools.cli :refer [parse-opts]]
            [clojure.string :as s]))
(set! js/XMLHttpRequest xhr2)

(def cli-options
  [["-h" "--help" "Show help"]
   ["-m" "--message" "You want to send message"]
   ["-t" "--token" "Your LINE API Token"]])

(defn usage
  [options-summary]
  (s/join \newline
          ["This is LIne Notify."
           ""
           "Usage: Line Notify [options] action."
           "Options:"
           options-summary
           ""]))

(defn parse-args
  [args]
  (map (fn [arg]
         (cond
           (or (= arg "-m") (= arg "--message")) :message
           (or (= arg "-t") (= arg "--token")) :token
           :else arg))
       args))

(defn split-args
  [args]
  (partition 2 (parse-args args)))

(defn arg-map
  [args]
  (into {} (map vec (split-args args))))

(defn send-message
  [token message]
  (go (let [response (<! (http/post "https://notify-api.line.me/api/notify"
                                    {:with-credentials? false?
                                     :headers {"Authorization"
                                               (str "Bearer "
                                                    token)}
                                     :form-params {:message message}}))]
        (prn (:status response))
        (prn (:body response)))))

(defn notify
  [args]
  (let [config (arg-map args)
        token (:token config)
        message (:message config)]
    (println config)
    (println token)
    (println message)
    (send-message token message)))

(defn -main
  [& args]
  (let [{:keys [options summary arguments]} (parse-opts args cli-options)]
    (cond
      (:help options) (println (usage summary))
      (and (:token options) (:message options)) (notify args)
      :else (println "this is line-notify!"))))
