(ns modoki.send_response
  (:import  [java.io FileInputStream])
  (:require [clojure.java.io :as io]
            [modoki.util :refer [write-line get-date-string-utc get-content-type]]))



(defn send-ok-response
  [output-stream fis ext]
  ;; response line
  (write-line output-stream "HTTP/1.1 200 OK")
  ;; response header
  (write-line output-stream (str  "Date: " (get-date-string-utc)))
  (write-line output-stream "Server: modoki")
  (write-line output-stream "Connection: close")
  (write-line output-stream (str "Content-type: " (get-content-type ext)))
  (write-line output-stream "")
  ;; response body
  (loop [ch (.read fis)]
    (when (not= ch -1)
      (.write output-stream ch)
      (.flush output-stream)
      (recur (.read fis)))))

(defn send-not-found-response
  [output-stream error-document]
  ;; response line
  (write-line output-stream "HTTP/1.1 404 Not Found")
  ;; response header
  (write-line output-stream (str  "Date: " (get-date-string-utc)))
  (write-line output-stream "Server: modoki")
  (write-line output-stream "Connection: close")
  (write-line output-stream (str "Content-type: text/html"))
  (write-line output-stream "")
  ;; response body
  (let [fis (io/input-stream (FileInputStream. error-document))]
    (loop [ch (.read fis)]
      (when (not= ch -1)
        (.write output-stream ch)
        (.flush output-stream)
        (recur (.read fis))))))
