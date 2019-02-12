(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream FileNotFoundException File]
           [java.nio.file FileSystems LinkOption])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all]
            [modoki.send_response :refer :all])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")
(def error-document-root "./resources/error")
(def server-name "localhost:8001")

(defn build-location
  [host path]
  (str "http://" (if (nil? host) server-name host) path))



(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          request-line (bytes->str (read-line input))
          path (get-request-path request-line)
          ext (get-ext path)
          host (get-host (bytes->str (read-line input)))]
      (println "request line: " request-line)
      (println "path: " path " "
               "ext: " ext " "
               "host: " host)
      (try
        (let [fis (io/input-stream (FileInputStream. (str document-root path)))
              fs (FileSystems/getDefault)
              path-obj (.getPath fs (str document-root path) (into-array [""]))
              real-path (.toRealPath path-obj (into-array LinkOption []))]
          (send-ok-response output fis ext))
        (catch FileNotFoundException ex
          (send-not-found-response output error-document-root))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
