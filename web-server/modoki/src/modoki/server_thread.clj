(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream FileNotFoundException File])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all]
            [modoki.send_response :refer [send-ok-response send-not-found-response]])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")
(def error-document-root "./resources/error")
(def server-name "localhost:8001")

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          request-line (bytes->str (read-line input))
          path (get-request-path request-line)
          ext (get-ext path)
          host (bytes->str (read-line input))]
      (println "request line: " request-line)
      (println "path: " path " "
               "ext: " ext " "
               "host: " host)
      (try
        (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
          (send-ok-response output fis ext))
        (catch FileNotFoundException ex
          (let [real-path (File. (str document-root path))]
            (if (.isDirectory real-path)
              (println "redirect to ...." (str document-root (redirect-path path)) )
              (send-not-found-response output error-document-root))))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
