(ns modoki.core
  (:import [java.net ServerSocket])
  (:require [modoki.server_thread :refer [server-thread]])
  (:gen-class))

(defn -main
  []
  (try
    (let  [server (ServerSocket. (Integer. 8001))
           ]
      (while true
        (.start (Thread. #(server-thread (.accept server))))))))
