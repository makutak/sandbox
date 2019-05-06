(ns henacat.servletimpl.http_servlet_request_impl
  (:import [java.net URLDecoder]
           [java.lang AssertionError]
           [java.io UnsupportedEncodingException]
           [java.nio.charset Charset])
  (:require [henacat.servletinterfaces.http_servlet_request :refer [HttpServletRequest]]
            [henacat.servletinterfaces.cookie :refer [make-Cookie]]
            [clojure.string :as s])
  (:refer-clojure :exclude [get-method]))

(defn parse-cookies
  [cookie-string]
  (if (nil? cookie-string)
    nil
    (do
      (let [cookie-pairs (s/split cookie-string #";")]
        (loop [ret []
               cookies cookie-pairs]
          (cond
              (empty? cookies) ret
              :else (let [pair (s/split (first cookies) #"=" 2)]
                      (recur
                       (conj ret (make-Cookie (first pair) (second pair)))
                       (rest cookies)))))))))

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
    @(:cookies this)))

(defn make-HttpServletRequestImpl
  [method request-header parameter-map]
  (HttpServletRequestImpl. method
                           (atom request-header)
                           parameter-map
                           (atom (parse-cookies (:COOKIE request-header)))))
