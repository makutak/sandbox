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

(defrecord Cookie [name value domain max-age path sercure http-only]
  ICookie
  (set-domain [this pattern]
    (reset! (:domain this) pattern))
  (get-domain [this]
    (:domain this))
  (set-max-age [this expiry]
    (reset! (:max-age this) expiry))
  (get-max-age [this]
    (:max-age this))
  (set-path [this uri]
    (reset! (:uri this) uri))
  (get-path [this]
    (:path this))
  (set-secure [this flag]
    (reset! (:sercure this) flag))
  (get-secure [this]
    (:secure this))
  (set-http-only [this http-only]
    (reset! (:http-only this) http-only))
  (is-http-only [this]
    (:http-only this))
  (get-name [this]
    (:name this))
  (set-value [this new-value]
    (reset! (:value this) new-value))
  (get-value [this]
    (:value this)))

(defn make-Cookie
  [name value]
  (Cookie. name
           value
           (atom nil) ;; domain
           (atom nil) ;; max-age
           (atom nil) ;; path
           (atom nil) ;; secure
           (atom nil) ;; http-only
           ))
