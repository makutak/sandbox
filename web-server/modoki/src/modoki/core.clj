(ns modoki.core
  (:import [java.net ServerSocket])
  (:import [java.io InputStream OutputStream])
  (:require [clojure.java.io :as io])
  (:refer-clojure :exclude [read-line write-line])
  (:gen-class))



;; (defn read-line
;;   [^InputStream input-stream]
;;   (let [input (.read input-stream)]
;;     (if-not (= input -1)
;;       (cons input (lazy-seq (read-line input-stream))))))

;; (defn write-line
;;   [^OutputStream output-stream string]
;;   (map #(.write output-stream (int %)) (char-array string)))

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
    (loop [ch (.read input)]
      (when-not (= ch -1)
        (.write output ch)
        (.flush output)
        (recur (.read input))))
    (println "done")
    (.close socket)))

(defn -main
  []
  (server (Integer. 8001)))
