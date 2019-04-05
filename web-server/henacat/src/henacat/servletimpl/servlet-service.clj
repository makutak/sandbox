(ns henacat.servletimpl.servlet-service
  (:import [java.nio.file FileSystems]
           [java.net URLClassLoader URL]
           [javax.servlet.http HttpServlet])
  (:require [clojure.java.io :refer [as-url]]))

(defn create-servlet
  [info]
  (let [fs (FileSystems/getDefault)
        path-obj (.getPath fs (:servlet-directory info) (into-array [""]))
        url (as-url (.toUri path-obj))
        loader (URLClassLoader/newInstance (into-array [url]))
        clazz (.loadClass loader (:servlet-className info))]
    (cast HttpServlet (.newInstance clazz))))
