(ns henacat.servletimpl.servlet_info
  (:import [java.util HashMap]))

(def servlet-collection (HashMap.))

(defrecord ServletInfo [url-pattern servlet-directory servlet-className servlet])

(defn make-ServletInfo
  "servlet-classname is jar file"
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-collection
        url-pattern
        (new ServletInfo url-pattern servlet-directory servlet-classname (atom nil))))

(defn add-servlet
  [url-pattern servlet-directory servlet-classname]
  (.put servlet-collection
        url-pattern
        (make-ServletInfo url-pattern servlet-directory servlet-classname)))

(defn search-servlet
  [url-pattern]
  (.get servlet-collection url-pattern))
