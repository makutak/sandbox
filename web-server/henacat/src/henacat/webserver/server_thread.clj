(ns henacat.webserver.server_thread
  (:import [java.net Socket URLDecoder]
           [java.io FileInputStream FileNotFoundException]
           [java.nio.file FileSystems Files LinkOption NoSuchFileException])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [henacat.util.util :as util]
            [henacat.util.send_response :as send_response]
            [henacat.servletimpl.servlet_info :refer [search-servlet]]
            [henacat.servletimpl.servlet_service :refer [do-service]]))

(def relative-document-root "./resources")
(def error-document-root "./resources/error")
(def server-name "localhost:8001")
(def file-system (FileSystems/getDefault))
(def default-char-set "UTF-8")
(def document-root
  (let [relative-path (.getPath file-system relative-document-root (into-array [""]))]
    (str (.toRealPath relative-path (into-array LinkOption [])))))


(defn add-request-header
  [request-header line]
  (println "servlet: " (search-servlet "/test-bbs/TestBBS"))
  (let [colon-pos (s/index-of line ":")]
    (if (not (nil? colon-pos))
      (assoc request-header
             (keyword (s/upper-case (subs line 0 colon-pos)))
             (s/trim (subs line (+ colon-pos 1))))
      request-header)))

(defn parse-request-header
  [input]
  (loop [line (util/bytes->str (util/read-line input))
         request-header {}]
    (if (not= line "\r")
      (let [res (add-request-header request-header line)]
        (recur (util/bytes->str (util/read-line input))
               res))
      request-header)))

(defn get-request-method
  [line]
  (cond
    (s/starts-with? line "GET") "GET"
    (s/starts-with? line "POST") "POST"))

(defn build-location
  [host path]
  (str "http://" (if (nil? host) server-name host) path))

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          output (io/output-stream socket)
          request-line (util/bytes->str (util/read-line input))
          method (get-request-method request-line)
          request-header (parse-request-header input)
          req-url (URLDecoder/decode (util/get-request-url request-line) default-char-set)
          path-and-query (s/split req-url #"\?")
          request-path (nth path-and-query 0)
          path (if (s/ends-with? request-path "/") (str request-path "index.html") request-path)
          ext (util/get-ext path)
          query (if (> (count path-and-query) 1) (nth path-and-query 1) nil)
          fs (FileSystems/getDefault)
          path-obj (.getPath fs (str document-root path) (into-array [""]))
          servlet-info (search-servlet request-path)
          ]
      (println "request-line: " request-line)
      (println "method: " method)
      (println "reqUrl: " req-url)
      (println "path: " path)
      (println "query: " query)
      (println "ext: " ext)
      (println "path-obj: " path-obj)
      (println "request-header: " request-header)
      (println "servlet-info: " servlet-info)
      (if (not (nil? servlet-info))
        (do-service method query servlet-info request-header input output)
        (do
          (try
            (let [real-path (.toRealPath path-obj (into-array LinkOption []))
                  location (build-location (:HOST request-header) path)]
              (println "real-path: " real-path)
              (println "location: " location)
              (cond
                (not (s/starts-with? real-path document-root))
                (do
                  (println "directory traversal")
                  (send_response/send-not-found-response output error-document-root))

                (Files/isDirectory real-path (into-array LinkOption []))
                (do
                  (println "redirect")
                  (send_response/send-move-permanently-response output location))

                :else
                (try
                  (let [fis (io/input-stream (FileInputStream. (str document-root path)))]
                    (println "ok!!!")
                    (send_response/send-ok-response output fis ext))
                  (catch FileNotFoundException ex
                    (do
                      (println "not found!!")
                      (send_response/send-not-found-response output error-document-root))))))
            (catch NoSuchFileException ex
              (do
                (println "error!!!")
                (send_response/send-not-found-response output error-document-root)))))))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
