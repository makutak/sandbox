(ns henacat.servletimpl.http_servlet_response_impl
  (:import [java.io OutputStreamWriter PrintWriter]
           [java.util ArrayList])
  (:require [henacat.util.send_response :refer [send-ok-response-header]]
            [henacat.servletinterfaces.http_servlet_response :refer [HttpServletResponse]]
            [clojure.string :as s]))

(def default-content-type "application/octet-stream")
(def default-character-encoding "UTF-8")

(defrecord HttpServletResponseImpl [content-type character-encoding output-stream print-writer cookies]
  HttpServletResponse
  (set-character-encoding [this charset]
    (reset! (:character-encoding this) charset))

  (set-content-type [this content-type]
    (reset! (:content-type this) content-type)
    (let [tmp (s/split content-type #" *;")]
      (if (> (count tmp) 1)
        (do
          (let [key-value (s/split (second tmp) #"=")]
            (if (and (= (count key-value) 2)
                     (= (first key-value) "charset"))
              (.set-character-encoding this (second key-value))))))))

  (get-writer [this]
    (reset! (:print-writer this) (PrintWriter. (OutputStreamWriter. (:output-stream this)
                                                                    @(:character-encoding this))))
    (send-ok-response-header @(:print-writer this) @(:content-type this))
    @(:print-writer this))

  (add-cookie [this cookie]
    (.add (:cookies this) cookie)))

(defn make-HttpServletResponseImpl
  [output]
  (HttpServletResponseImpl. (atom default-content-type)
                            (atom default-character-encoding)
                            output
                            (atom nil)
                            (ArrayList.)))
