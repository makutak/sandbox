(ns henacat.servletimpl.session_manager
  (:import [java.util.concurrent ConcurrentHashMap])
  (:require [henacat.servletimpl.http_session_impl :refer [make-HttpSessionImpl]]))
