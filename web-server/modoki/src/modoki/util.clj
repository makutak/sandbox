(ns modoki.util
  (:import [java.io InputStream OutputStream]
           [java.util Date Calendar TimeZone Locale]
           [java.text SimpleDateFormat])
  (:require [clojure.string :as s])
  (:refer-clojure :exclude [read-line write-line]))

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
  (->> (filter #(not= % \return) lat)
       (map #(char %))
       (apply str)))

(defn get-request-path
  [request-line]
  (second (s/split request-line #" ")))

(defn get-date-string-utc
  []
  (let [cal (Calendar/getInstance (TimeZone/getTimeZone "GMT"))
        df (SimpleDateFormat. "EEE, dd MMM yyyy HH:mm:ss" Locale/US)]
    (.setTimeZone df (.getTimeZone cal))
    (str  (.format df (.getTime cal)) " GMT")))

(defn redirect-path
  [path]
  (str path "index.html"))

(defn get-ext
  [path]
  (last (s/split path #"\.")))

(defn get-host
  [host]
  (s/trim (second (s/split host #" "))))
