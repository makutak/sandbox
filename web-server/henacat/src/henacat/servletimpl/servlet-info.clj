(ns henacat.servletimpl.servlet-info
  (:import [java.util HashMap]))

(def servlet-collection (HashMap.))
(def servlet-info (HashMap.))

(defn make-servlet-info
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-info url-pattern url-pattern)
  (.put servlet-info servlet-directory servlet-directory)
  (.put servlet-info servlet-classname servlet-classname)
  servlet-info)

(defn add-servlet
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-collection
        url-pattern
        (make-servlet-info url-pattern servlet-directory servlet-classname)))

(defn search-servlet
  [url-pattern]
  (.get servlet-collection url-pattern))
