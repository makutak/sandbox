(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream FileNotFoundException])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all]
            [modoki.send_response :refer [send-ok-response send-not-found-response]])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          path (get-path (bytes->str (read-line input)))
          ext (last (s/split path #"\."))]
      (println "path: " path " " "ext:" ext)
      (try
        (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
          (send-ok-response output fis ext))
        (catch FileNotFoundException ex
          (send-not-found-response output))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
