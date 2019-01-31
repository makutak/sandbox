(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [modoki.server :refer [server-thread]])
  (:gen-class))

(defn -main
  []
  (try
    (let  [server (ServerSocket. (Integer. 8001))
           socket (.accept server)]
      (while true
        (.start (Thread. #(server-thread socket)))))))
