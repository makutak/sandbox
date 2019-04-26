(ns henacat.servletimpl.http_servlet_response_impl
  (:import [java.io OutputStreamWriter PrintWriter])
  (:require [henacat.util.send_response :refer [send-ok-response-header]]
            [henacat.servletinterfaces.http_servlet_response :refer [HttpServletResponse]]
            [clojure.string :as s]))

(def default-content-type "application/octet-stream")
(def default-character-encoding "UTF-8")

(defrecord HttpServletResponseImpl [content-type character-encoding output-stream print-writer]
  HttpServletResponse
  (set-character-encoding [this charset]
    (reset! (:character-encoding this) charset))

  (set-content-type [this content-type]
    (reset! (:content-type this) content-type)
    (let [tmp (s/split content-type #" *;")]
      (if (> (count tmp) 1)
        (let [key-value (s/split (second tmp) #"=")]
          (if (and (= (count key-value) 2)
                   (= (first key-value) "charset"))
            (.set-character-encoding this (second key-value)))))))

  (get-writer [this]
    (reset! (:print-writer this) (PrintWriter. (OutputStreamWriter. (:output-stream this)
                                                                    @(:character-encoding this))))
    (send-ok-response-header @(:print-writer this) @(:content-type this))
    @(:print-writer this)))

(defn make-HttpServletResponseImpl
  [output]
  (new HttpServletResponseImpl (atom nil) (atom nil) output (atom nil)))
