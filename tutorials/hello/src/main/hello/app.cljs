(ns hello.app
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]))

(set! js/XMLHttpRequest xhr2)

(def token "<Line Api Token>")

(defn test-request
  []
  (go (let [response (<! (http/get "https://example.com"))]
        (prn (:status response))
        (prn (:body response)))))

(defn notify
  []
  (go (let [response (<! (http/post "https://notify-api.line.me/api/notify"
                                    {:with-credentials? false?
                                     :headers {"Authorization"
                                               (str "Bearer "
                                                    token)}
                                     :form-params {:message "Hello ClojureScript!!"}}))]
        (prn (:status response))
        (prn (:body response)))))

(defn -main
  [& args]
  (println args))
