(ns hello.app
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require [cljs.core.async :refer [<!]]
            [cljs-http.client :as http]))

(defn test-request
  []
  (go (let [response (<! (http/get "https://example.com"))]
        (prn (:status response)))))


(defn -main
  [& args]
  (println args))
