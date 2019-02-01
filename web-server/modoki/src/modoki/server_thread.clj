(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")

(defn server-thread
  [^Socket socket]
  (println "connect!!")
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          path (get-path (bytes->str (read-line input)))
          ext (last (s/split path #"\."))]
      (println "path: " path)
      (println "ext: " ext )
      ;; response line
      (write-line output "HTTP/1.1 200 OK")
      ;; response header
      (write-line output (str  "Date: " (get-date-string-utc)))
      (write-line output "Server: modoki")
      (write-line output "Connection: close")
      (write-line output (str "Content-type: " (get-content-type ext)))
      (write-line output "")
      ;; response body
      (if (> (count path) 1)
        (do
          (try
            (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
              (loop [ch (.read fis)]
                (when (not= ch -1)
                  (.write output ch)
                  (.flush output)
                  (recur (.read fis)))))
            (catch Exception e
              (.printStackTrace e))))
        (write-line output "<h1>Hello Wolrd!!</h1>")))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
