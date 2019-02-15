(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [modoki.server_thread :refer [server-thread]])
  (:gen-class))

(defn -main
  []
  (try
    (let [server (ServerSocket. (Integer. 8001))]
      (println "wait for connect..")
      (while true
        (let [socket (.accept server)]
          (.start (Thread. #(server-thread socket))))))))
