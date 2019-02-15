(ns modoki.server_thread
  (:import [java.net Socket URLDecoder]
           [java.io FileInputStream FileNotFoundException]
           [java.nio.file FileSystems Files LinkOption NoSuchFileException])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [modoki.util :refer :all]
            [modoki.send_response :refer :all])
  (:refer-clojure :exclude [read-line write-line]))

(def relative-document-root "./resources")
(def error-document-root "./resources/error")
(def server-name "localhost:8001")
(def file-system (FileSystems/getDefault))
(def default-char-set "UTF-8")
(def document-root
  (let [relative-path (.getPath file-system relative-document-root (into-array [""]))]
    (str (.toRealPath relative-path (into-array LinkOption [])))))

(defn build-location
  [host path]
  (str "http://" (if (nil? host) server-name host) path))

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          request-line (bytes->str (read-line input))
          path (URLDecoder/decode (get-request-path request-line) default-char-set)
          ext (get-ext path)
          host (get-host (bytes->str (read-line input)))
          fs (FileSystems/getDefault)
          path-obj (.getPath fs (str document-root path) (into-array [""]))
          ]
      (println "request line: " request-line)
      (println "path: " path " "
               "ext: " ext " "
               "host: " host)
      (try
        (let [real-path (.toRealPath path-obj (into-array LinkOption []))
              location (build-location host (redirect-path path))]
          (cond
            ;; ドキュメントルートの外を見ようとしているとき
            (not (s/starts-with? real-path document-root))
            (send-not-found-response output error-document-root)
            ;; パスがディレクトリか
            (Files/isDirectory real-path (into-array LinkOption []))
            (send-move-permanently-response output location)
            ;; パスが正しい
            :else (try
                    (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
                      (send-ok-response output fis ext))
                    (catch FileNotFoundException ex
                      (send-not-found-response output error-document-root)))))
        (catch NoSuchFileException ex
          (send-not-found-response output error-document-root))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
