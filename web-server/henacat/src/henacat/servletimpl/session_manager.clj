(ns henacat.servletimpl.session_manager
  (:import [java.util.concurrent ConcurrentHashMap])
  (:require [henacat.servletimpl.http_session_impl :refer [make-HttpSessionImpl]]
            [henacat.servletimpl.session_id_generator :refer [generate-session-id]]))

(def CLEAN_INTERVAL 60)
(def SESSION_TIMEOUT 10)

(def sessions (ConcurrentHashMap.))
