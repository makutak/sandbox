(ns henacat.servletimpl.server_thread
  (:import [java.net Socket])
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [henacat.util.util :as util]))

(defn server-thread
  [^Socket socket]
  (try
    (let [input (io/input-stream socket)
          request-line (util/bytes->str (util/read-line input))
          ;; TODO: request methodを取得

          ])
    (println request-line)
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
