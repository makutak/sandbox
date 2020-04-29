(ns clj-scraper.core
  (:require [net.cgrand.enlive-html :as html]
            [org.httpkit.client :as http]))

(defn get-dom
  []
  (Thread/sleep 1000)
  (html/html-snippet
   (:body @(http/get "http://kmaebashi.com/") {:insecure? true})))

(defn extract-titles
  [dom]
  (map
   (comp first :content) (html/select dom [:a])))

(defn -main
  []
  (let [titles (extract-titles (get-dom))]
    (println titles)))
