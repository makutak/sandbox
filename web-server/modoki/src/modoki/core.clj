(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/reader socket)]
    (println "connect!!")
    (loop [r (.readLine input)]
      (when (not (nil? r))
        (println r)
        (recur (.readLine input))))
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
