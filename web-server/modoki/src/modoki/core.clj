(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/input-stream socket)]
    (println "connect!!")
    (let [in (.readLine (io/reader socket))]
      (println socket)
      (println in))))

(defn -main
  []
  (server (Integer. 8001)))
