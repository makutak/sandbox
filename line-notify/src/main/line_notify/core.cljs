(ns line_notify.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]
            [clojure.tools.cli :refer [parse-opts]]
            [clojure.string :as s]
            [cljs-node-io.core :as io]
            [cljs-node-io.fs :as fs]))
(set! js/XMLHttpRequest xhr2)

(declare set-token
         make-config-dir
         read-token-and-send
         notify
         send-message)

(def cli-options
  [["-h" "--help" "Show help"]
   ["-m" "--message" "You want to send message."]
   ["-t" "--token" "Your LINE API Token."]
   ["-s" "--set" "Your LINE API Token that you want to set config file."]])

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
                                               (str "Bearer " token)}
                                     :form-params {:message message}}))]
        (prn (:body response)))))

(defn notify
  [args]
  (let [config (arg-map args)
        token (:token config)
        message (:message config)]
    (send-message token message)))

(defn make-config-dir
  [token]
  (go
    (let [[err] (<! (fs/amkdir "config"))]
      (if-not err
        (set-token token)
        (println err)))))

(defn set-token
  [token]
  (go
    (let [[err] (<! (io/aspit "config/token.json" (.stringify js/JSON (clj->js {:token token})) ))]
      (if-not err
        (println "you've successfully written to 'token.json'")
        (make-config-dir token)))))

(defn read-token-and-send
  [message]
  (go
    (let [[err res] (<! (io/aslurp "config/token.json"))]
      (if-not err
        (send-message (:token (js->clj (.parse js/JSON res) :keywordize-keys true)) message)
        (println err)))))

(defn -main
  [& args]
  (let [{:keys [options summary arguments]} (parse-opts args cli-options)]
    (cond
      (:help options) (println (usage summary))
      (and (:set options) (not (empty? arguments))) (set-token (first arguments))
      (and (:token options) (:message options)) (notify args)
      (and (:message options) (not (empty? arguments))) (read-token-and-send (first arguments))
      :else (println (usage summary)))))
