(ns henacat.webserver.core
  (:import [java.net ServerSocket])
  (:require [henacat.webserver.server_thread :refer [server-thread]]
            [henacat.servletimpl.servlet_info :refer [add-servlet]])
  (:gen-class))

(defn -main
  []
  (add-servlet "/test-bbs/TestBBS"
               "../test-bbs/target/test-bbs-0.1.0-SNAPSHOT-standalone.jar"
               "TestBBS")
  (try
    (let [server (ServerSocket. (Integer. 8001))]
      (println "Waiting...")
      (while true
        (let [socket (.accept server)]
          (.start (Thread. #(server-thread socket))))))))
