(ns modoki.core
  (:import [java.net ServerSocket])
  (:import [java.io PrintStream])
  (:require [clojure.java.io :as io])
  (:refer-clojure :exclude [read-line])
  (:gen-class))

(defn read-line
  [input]
  )


(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (.getInputStream socket)]
    (println"connect!!")
    (loop [ch (.read input)]
      (when-not (= ch 0)
        (println ch)
        (recur (.read input))))))

(defn -main
  []
  (server (Integer. 8001)))
