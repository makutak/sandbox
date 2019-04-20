(ns henacat.servletinterfaces.http_servlet_request
  (:refer-clojure :exclude [get-method]))

(defprotocol HttpServletRequest
  (get-method [this])
  (get-parameter [this key-name])
  (set-character-encoding [this env]))
