(ns henacat.servletinterfaces.http_session)

(defprotocol HttpSession
  (get-id [this])
  (get-attribute [this attribute-name])
  (get-attribute-names [this])
  (remove-attribute [this attribute-name])
  (set-attribute [this attribute-name attribute-value]))
