(ns henacat.servletimpl.http_session_impl
  (:import [java.util.concurrent ConcurrentHashMap])
  (:require [henacat.servletinterfaces.http_session :refer [HttpSession]]))

(defrecord HttpSessionImpl [id attributes last-accessed-time]
  HttpSession
  (get-id [this]
    (:id this))

  (get-attribute [this attribute-name]
    (.get (:attributes this) attribute-name))

  (get-attribute-names [this]
    (.keySet (:attributes this)))

  (remove-attribute [this attribute-name]
    (.remove (:attributes this) attribute-name))

  (set-attribute [this attribute-name attribute-value]
    (if (nil? attribute-value)
      (.remove-attribute this attribute-name)
      (.put (:attributes this) attribute-name attribute-value))))

(defn make-HttpSessionImpl
  [id]
  (HttpSessionImpl. id
                    (ConcurrentHashMap.)
                    (atom (System/currentTimeMillis))))
