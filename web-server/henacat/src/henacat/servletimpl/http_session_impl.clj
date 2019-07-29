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

(defmulti access class)
(defmethod access HttpSessionImpl [http-session-impl]
  (locking http-session-impl (reset! (:last-accessed-time http-session-impl)
                                     (System/currentTimeMillis))))


(defn make-HttpSessionImpl
  [id]
  (let [session (HttpSessionImpl. id
                                  (ConcurrentHashMap.)
                                  (atom nil))]
    (access session)
    session))
