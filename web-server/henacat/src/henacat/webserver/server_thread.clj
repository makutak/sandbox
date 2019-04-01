(ns henacat.webserver.server_thread
  (:import [java.net Socket URLDecoder])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [henacat.util.util :as util]))

(def default-char-set "UTF-8")

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          request-line (util/bytes->str (util/read-line input))
          req-url (URLDecoder/decode (util/get-request-url request-line) default-char-set)
          path-and-query (s/split req-url #"\?")
          path (nth path-and-query 0)
          ext (util/get-ext path)
          query (if (> (count path-and-query) 1) (nth path-and-query 1) nil)
          output (io/output-stream socket)
          ;; TODO: request methodを取得
          ]

      (println "request-line: " request-line)
      (println "reqUrl: " req-url)
      (println "path: " path)
      (println "query: " query)
      (println "ext: " ext))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
