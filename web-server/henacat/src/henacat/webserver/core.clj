(ns henacat.webserver.core
  (:import [java.net ServerSocket])
  (:require [henacat.servletimpl.server_thread :refer [server-thread]])
  (:gen-class))

(defn -main
  []
  (try
    (let [server (ServerSocket. (Integer. 8001))]
      (println "Waiting...")
      (while true
        (let [socket (.accept server)]
          (.start (Thread. #(server-thread socket))))))))
