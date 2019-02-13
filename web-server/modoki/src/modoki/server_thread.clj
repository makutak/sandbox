(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream FileNotFoundException]
           [java.nio.file FileSystems Files LinkOption])
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
              ]
          (send-ok-response output fis ext))
        (catch FileNotFoundException ex
          (let [fs (FileSystems/getDefault)
                path-obj (.getPath fs (str document-root path) (into-array [""]))
                location (build-location host (redirect-path path))]
            (if (Files/isDirectory path-obj (into-array LinkOption []))
              (send-move-permanently-response output location)
              (send-not-found-response output error-document-root))))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
