(ns henacat.servletimpl.server_thread
  (:import [java.net Socket])
  (:require [clojure.java.io :as io]
            [clojure.string :as s])
  (:refer-clojure :exclude [read-line write-line]))

(defn server-thread
  [^Socket socket]
  (try
    (println socket)
    (catch Exception e
      (.printStackTrace e))
    (finally
      (.close socket))))
