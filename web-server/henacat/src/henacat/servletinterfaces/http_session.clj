(ns henacat.servletinterfaces.http_session)

(defprotocol HttpSession
  (get-id [this])
  (get-attribute [this name])
  (get-attribute-names [this])
  (remove-attribute [this name])
  (set-attribute [this value]))
