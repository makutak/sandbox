(ns post.core
  (:import [java.net ServerSocket]
           [java.io File FileInputStream FileOutputStream])
  (:gen-class))

(defn server
  [port]
  (println "クライアントからの接続を待ちます。")
  (try
    (let [server (ServerSocket. port)
          socket (.accept server)
          fos (FileOutputStream. "multipart_form-data_server_recv.txt")
          input (.getInputStream socket)]
      (loop [ch (.read input)]
        (when (not= ch -1)
          (.write fos ch)
          (recur (.read input))))
      (.close socket))
    (catch Exception e
      (.printStackTrace e))))

(defn -main
  []
  (server (Integer. 8001)))
