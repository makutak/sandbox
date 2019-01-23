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
    (let [aaa (slurp input)]
      (println aaa)
      (str aaa))
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
