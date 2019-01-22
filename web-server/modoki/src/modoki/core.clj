(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn server
  [port-number]
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/input-stream socket)]
    (println input)))

(defn -main
  "I don't do a whole lot ... yet."
  []
  (server (Integer. 8001)))
