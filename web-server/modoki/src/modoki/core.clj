(ns modoki.core
  (:import [java.net ServerSocket]
           [java.io InputStream OutputStream]
           [java.util Date Calendar TimeZone Locale]
           [java.text SimpleDateFormat])
  (:require [clojure.java.io :as io]
            [clojure.string :as s])
  (:refer-clojure :exclude [read-line write-line])
  (:gen-class))

(def document-root "./")

(defn read-line
  [^InputStream input-stream]
  (let [input (.read input-stream)]
    (if (and (not (= input -1))
             (not (= input (int \newline))))
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

(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/input-stream socket)
              output (io/output-stream socket)]
    (println "connect!!")

    (let [path (get-path (bytes->str (read-line input)))]
      (println "path: " path)
      ;; response header
      (write-line output "HTTP/1.1 200 OK")
      (write-line output (str  "Date: " (getDateStringUtc)))
      (write-line output "Server: modoki")
      (write-line output "Connection: close")
      (write-line output "Content-type: text/html")
      (write-line output "")
      ;; response body
      (try
        (write-line output "<h1>Hello World</h1>")
        (catch Exception e
          (.printStackTrace e))))
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
