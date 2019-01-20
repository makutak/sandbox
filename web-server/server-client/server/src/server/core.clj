(ns server.core
  (:import [java.net ServerSocket]
           [java.io File FileInputStream FileOutputStream])
  (:gen-class))

(defn server
  [port-number]
  (try
    (let [server (ServerSocket. port-number)
          fos (FileOutputStream. "server_recv.txt")
          fis (FileInputStream. "server_send.txt")]
      (println "クライアントからの接続を待ちます。")
      (let [socket (.accept server)
            input (.getInputStream socket)
            output (.getOutputStream socket)]
        (println "クライアント接続。")
        (loop [ch (.read input)]
          (when (not (= 0 ch))
            (.write fos ch)
            (recur (.read input))))
        (loop [ch (.read fis)]
          (when (not (= -1 ch))
            (.write output ch)
            (recur (.read fis))))
        (.close socket)))
    (catch Exception ex
      (.printStackTrace ex))))

(defn -main
  "I don't do a whole lot ... yet."
  []
  (server (Integer. 8001)))
