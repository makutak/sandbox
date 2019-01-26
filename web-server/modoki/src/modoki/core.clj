(ns modoki.core
  (:import [java.net ServerSocket])
  (:import [java.io InputStream OutputStream])
  (:import [java.io InputStream OutputStream])
  (:require [clojure.java.io :as io])
  (:refer-clojure :exclude [read-line write-line])
  (:gen-class))



;; (defn read-line
;;   [^InputStream input-stream]
;;   (let [input (.read input-stream)]
;;     (if-not (= input -1)
;;       (cons input (lazy-seq (read-line input-stream))))))

(defn write-line
  [^OutputStream output-stream string]
  ;;(map #(.flush (.write output-stream (int %))) (char-array string))
  (doseq [ch (char-array string)]
    (.write output-stream (int ch)))
  (.write output-stream (int \return))
  (.write output-stream (int \newline))
  (.flush output-stream))

(defn server
  [port-number]
  (println "wait for connect..")
  (with-open [server (ServerSocket. port-number)
              socket (.accept server)
              input (io/input-stream socket)
              output (io/output-stream socket)]
    (println"connect!!")
    ;;(dorun (map #(.write output %) (read-line input)))
    ;; (dorun (lazy-seq (write-line output "test")))
    ;; (loop [ch (.read input)]
    ;;   (when (not (= ch -1))
    ;;     (recur (.read input))))
    ;; (println "loop done")

    ;; header
    (write-line output "HTTP/1.1 200 OK")
    (write-line output "Date: Sat, 26 Jan 2019 17:35:18 GMT")
    (write-line output "Server: modoki")
    (write-line output "Connection: close")
    (write-line output "Content-type: text/html")
    (write-line output "")
    ;; body
    (try
      (write-line output "<h1>It works</1>")
      (catch Exception e
        (.printStackTrace e)))
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
