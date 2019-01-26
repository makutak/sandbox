(ns modoki.core
  (:import [java.net ServerSocket])
  (:import [java.io InputStream OutputStream])
  (:require [clojure.java.io :as io])
  (:refer-clojure :exclude [read-line write-line])
  (:gen-class))



(defn read-line
  [^InputStream input-stream]
  (let [input (.read input-stream)]
    (if-not (= input -1)
      (cons input (lazy-seq (read-line input-stream))))))

(defn write-line
  [^OutputStream output-stream string]
  (doseq [ch (map #(int %) (char-array string))]
    (.write output-stream ch)))

(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/input-stream socket)
              output (.getOutputStream socket)]
    (println"connect!!")
    (dorun (map #(.write output %) (read-line input)))
    (.flush output)
    (println "done")
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
