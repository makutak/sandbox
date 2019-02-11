(ns modoki.send_response
  (:import  [java.io FileInputStream])
  (:require [clojure.java.io :as io]
            [modoki.util :refer [write-line get-date-string-utc get-content-type]]))

(def not-found-page "/404.html")

(defn send-ok-response
  [output-stream fis ext]
  ;; response line
  (write-line output-stream "HTTP/1.1 200 OK")
  ;; response header
  (write-line output-stream (str "Date: " (get-date-string-utc)))
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
  [output-stream error-document-root]
  ;; response line
  (write-line output-stream "HTTP/1.1 404 Not Found")
  ;; response header
  (write-line output-stream (str "Date: " (get-date-string-utc)))
  (write-line output-stream "Server: modoki")
  (write-line output-stream "Connection: close")
  (write-line output-stream (str "Content-type: text/html"))
  (write-line output-stream "")
  ;; response body
  (let [fis (io/input-stream (FileInputStream. (str error-document-root not-found-page)))]
    (loop [ch (.read fis)]
      (when (not= ch -1)
        (.write output-stream ch)
        (.flush output-stream)
        (recur (.read fis))))))

(defn send-move-permanently-response
  [output-stream location]
  (write-line output-stream "HTTP/1.1 301 Moved Permanently")
  (write-line output-stream (str "Date: " (get-date-string-utc)))
  (write-line output-stream "Server: modoki")
  (write-line output-stream (str "Location: " location))
  (write-line output-stream "Connection: close")
  (write-line output-stream ""))
