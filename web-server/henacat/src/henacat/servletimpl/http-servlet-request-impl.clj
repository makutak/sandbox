(ns henacat.servletimpl.http-servlet-request-impl
  (:import [java.net URLDecoder]
           [java.lang AssertionError]
           [java.io UnsupportedEncodingException]
           [java.nio.charset Charset]))

(defprotocol IHttpServletRequestImpl
  (get-method [this])
  (get-parameter [this key-name])
  (set-character-encoding [this env]))

(defrecord HttpServletRequestImpl [method character-encoding parameter-map]
  IHttpServletRequestImpl
  (get-method [this]
    (:method this))
  (get-parameter [this key-name]
    (let [value ((keyword key-name) (:parameter-map this))]
      (try
        (let [decoded (URLDecoder/decode value (:character-encoding this))]
          decoded)
        (catch UnsupportedEncodingException ex
          (throw (AssertionError. ex))))))
  (set-character-encoding [this env]
    (if (not (Charset/isSupported env))
      (throw (UnsupportedEncodingException. (str "encoding. " env)))
      (reset! (:character-encoding env)))))

(defn make-HttpServletRequestImpl
  [method parameter-map]
  (new HttpServletRequestImpl method (atom nil) parameter-map))
