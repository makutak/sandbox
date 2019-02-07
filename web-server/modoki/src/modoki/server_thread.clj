(ns modoki.server_thread
  (:import [java.net Socket]
           [java.io FileInputStream FileNotFoundException])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all]
            [modoki.send_response :refer [send-ok-response send-not-found-response]])
  (:refer-clojure :exclude [read-line write-line]))

(def document-root "./resources")
(def error-document-root "./resources/error")

(defn read-request
  [input]
  (loop [ch input
         request '()]
    (let [line (read-line ch)]
      (if (= (bytes->str line) )
        request
        (recur input
               (cons line request))))))


(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          request (map #(bytes->str %) (read-request input))
          ;; output (io/output-stream socket)
          ;; path (get-path (bytes->str (read-line input)))
          ;; ext (last (s/split path #"\."))
          ]
      ;;(println "path: " path " " "ext:" ext)
      (println "request...")
      (println (first (reverse request)))
      (println (second (reverse request)))
      ;;(println  (map #(bytes->str %)w )
      ;; (try
      ;;   (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
      ;;     (send-ok-response output fis ext))
      ;;   (catch FileNotFoundException ex
      ;;     (send-not-found-response output error-document-root)))
      )
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
