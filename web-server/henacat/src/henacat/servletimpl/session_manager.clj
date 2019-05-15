(ns henacat.servletimpl.session_manager
  (:import [java.util.concurrent ConcurrentHashMap Executors ScheduledExecutorService TimeUnit])
  (:require [henacat.servletimpl.http_session_impl :refer [make-HttpSessionImpl access]]
            [henacat.servletimpl.session_id_generator :refer [generate-session-id]]))

(declare cleaner-handle
         clean-sessions)

(def CLEAN_INTERVAL 60)
(def SESSION_TIMEOUT 10)

(def ^:dynamic *sessions* (ConcurrentHashMap.))
(def session-id-generater (generate-session-id))
(def scheduler
  (let [scheduler (Executors/newSingleThreadScheduledExecutor)]
    (.scheduleWithFixedDelay scheduler
                             #(cleaner-handle)
                             CLEAN_INTERVAL
                             CLEAN_INTERVAL
                             TimeUnit/SECONDS)))

(def cleaner-handle scheduler)

(defn get-session
  [id]
  (let [ret (.get *sessions* (keyword id))]
    (when (not (nil? ret))
      (access ret))
    ret))

(defn create-session
  []
  (let [id (generate-session-id)
        session (make-HttpSessionImpl id)]
    (.put *sessions* (keyword id) session)
    session))

(defn clean-sessions
  []
  scheduler)

(defn session-manager
  []
  cleaner-handle)
