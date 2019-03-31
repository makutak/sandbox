(ns henacat.servletimpl.server_thread
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
          path (URLDecoder/decode (util/get-request-path request-line) default-char-set)
          ;; TODO: request methodを取得

          ]
      (println request-line)
      (println path))
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
