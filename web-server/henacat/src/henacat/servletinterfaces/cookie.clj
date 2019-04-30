(ns henacat.servletinterfaces.cookie)

(defprotocol ICookie
  (set-domain [this pattern])
  (get-domain [this])
  (set-max-age [this expiry])
  (get-max-age [this])
  (set-path [this uri])
  (get-path [this])
  (set-secure [this flag])
  (get-secure [this])
  (set-http-only [this http-only])
  (is-http-only [this])
  (get-name [this])
  (set-value [this new-value])
  (get-value [this]))

(defrecord Cookie [name value domain max-age path sercure http-only])
