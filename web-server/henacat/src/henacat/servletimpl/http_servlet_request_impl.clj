(ns henacat.servletimpl.http_servlet_request_impl
  (:import [java.net URLDecoder]
           [java.lang AssertionError]
           [java.io UnsupportedEncodingException]
           [java.nio.charset Charset])
  (:require [henacat.servletinterfaces.http_servlet_request :refer [HttpServletRequest]])
  (:refer-clojure :exclude [get-method]))

(defrecord HttpServletRequestImpl [method character-encoding parameter-map cookies]
  HttpServletRequest
  (get-method [this]
    (:method this))

  (get-parameter [this key-name]
    (let [value ((keyword key-name) (:parameter-map this))]
      (try
        (let [decoded (URLDecoder/decode value @(:character-encoding this))]
          decoded)
        (catch UnsupportedEncodingException ex
          (throw (AssertionError. ex))))))

  (set-character-encoding [this env]
    (if (not (Charset/isSupported env))
      (throw (UnsupportedEncodingException. (str "encoding. " env)))
      (reset! (:character-encoding this) env)))
  (get-cookies [this]
    (:cookies this)))

(defn make-HttpServletRequestImpl
  [method parameter-map]
  (HttpServletRequestImpl. method
                           (atom nil)
                           parameter-map
                           (atom nil)))
