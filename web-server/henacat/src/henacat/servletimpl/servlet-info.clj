(ns henacat.servletimpl.servlet-info
  (:import [java.util HashMap]))

(def servlet-collection (HashMap.))

(defrecord ServletInfo [url-pattern servlet-directory servlet-className servlet])

(defn make-servlet-info
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-collection
        url-pattern
        (new ServerInfo url-pattern servlet-directory servlet-classname nil)))

(defn add-servlet
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-collection
        url-pattern
        (make-servlet-info url-pattern servlet-directory servlet-classname)))

(defn search-servlet
  [url-pattern]
  (.get servlet-collection url-pattern))
