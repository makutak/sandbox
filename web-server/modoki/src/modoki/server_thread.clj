(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io InputStream OutputStream FileInputStream]
           [java.util Date Calendar TimeZone Locale]
           [java.text SimpleDateFormat])
  (:require [clojure.java.io :as io]
            [clojure.string :as s])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")

(def content-type-map {:html "text/html"
                       :htm "text/html"
                       :css "text/css"
                       :png "image/png"
                       :jpg "image/jpeg"
                       :jpeg "image/jpeg"
                       :gif "image/gif"})

(defn get-content-type
  [ext]
  ((keyword ext) content-type-map "application/octet-stream"))

(defn read-line
  [^InputStream input-stream]
  (let [input (.read input-stream)]
    (if (and (not= input -1)
             (not= input (int \newline)))
      (cons input (read-line input-stream)))))

(defn write-line
  [^OutputStream output-stream string]
  (doseq [ch (char-array string)]
    (.write output-stream (int ch)))
  (.write output-stream (int \return))
  (.write output-stream (int \newline))
  (.flush output-stream))

(defn bytes->str
  [lat]
  (apply str (map #(char %) lat)))

(defn get-path
  [request-line]
  (second (s/split request-line #" ")))

(defn getDateStringUtc
  []
  (let [cal (Calendar/getInstance (TimeZone/getTimeZone "GMT"))
        df (SimpleDateFormat. "EEE, dd MMM yyyy HH:mm:ss" Locale/US)]
    (.setTimeZone df (.getTimeZone cal))
    (str  (.format df (.getTime cal)) " GMT")))

(defn server-thread
  [^Socket socket]
  (println "connect!!")
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          path (get-path (bytes->str (read-line input)))]
      (println "path: " path)
      ;; response line
      (write-line output "HTTP/1.1 200 OK")
      ;; response header
      (write-line output (str  "Date: " (getDateStringUtc)))
      (write-line output "Server: modoki")
      (write-line output "Connection: close")
      (write-line output "Content-type: text/html")
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
