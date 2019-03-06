(ns hello.app
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require ["xhr2" :as xhr2]
            [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]))

(set! js/XMLHttpRequest xhr2)


(defn test-request
  []
  (go (let [response (<! (http/get "https://example.com"))]
        (prn (:status response))
        (prn (:body response)))))


(defn -main
  [& args]
  (println args))
