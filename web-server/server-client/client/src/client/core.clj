(ns client.core
  (:import [java.io FileInputStream FileOutputStream]
           [java.net Socket])
  (:gen-class))

(defn client
  [port-number]
  (try
     (let [socket (Socket. "localhost" port-number)
           fis (FileInputStream. "client_send.txt")
           fos (FileOutputStream. "client_recv.txt")
           input (.getInputStream socket)
           output (.getOutputStream socket)]
       (loop [ch (.read fis)]
         (when (not (= ch -1))
           (.write output ch)
           (recur (.read fis))))
       ;; (.write output 0)
       (loop [ch (.read input)]
         (when (not (= ch -1))
           (.write fos ch)
           (recur (.read input)))))
    (catch Exception ex
      (.printStackTrace ex))))

(defn -main
  "I don't do a whole lot ... yet."
  []
  (client (Integer. 8001)))
