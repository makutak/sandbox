(ns henacat.servletimpl.servlet-service
  (:import [java.nio.file FileSystems]
           [java.net URLClassLoader URL]
           [javax.servlet.http HttpServlet]
           [java.util HashMap]
           [java.lang StringBuilder])
  (:require [clojure.java.io :refer [as-url]]
            [clojure.string :as s]))

(defn create-servlet
  [info]
  (let [fs (FileSystems/getDefault)
        path-obj (.getPath fs (:servlet-directory info) (into-array [""]))
        url (as-url (.toUri path-obj))
        loader (URLClassLoader/newInstance (into-array [url]))
        clazz (.loadClass loader (:servlet-className info))]
    (cast HttpServlet (.newInstance clazz))))

(defn string->map
  [string]
  (let [parameter-map {}]
    (if (not (empty? string))
      (reduce
       (fn [param-map param]
         (let [key-value (s/split param #"=")]
           (assoc param-map (keyword (first key-value)) (last key-value))))
       parameter-map
       (s/split  string #"\&"))
      parameter-map)))

(defn read->size
  [input size]
  (loop [ch (.read input)
         sb (StringBuilder.)
         read-size 0]
    (when (and (< read-size size)
               (not (= -1 ch)))
      (.append sb (char ch))
      (inc read-size))
    (.toString sb)))

(defn doService
  [method query info request-header input output])
