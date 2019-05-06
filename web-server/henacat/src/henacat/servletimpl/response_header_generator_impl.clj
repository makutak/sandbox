(ns henacat.servletimpl.response_header_generator_impl
  (:import [java.text SimpleDateFormat]
           [java.util Date Calendar TimeZone Locale])
  (:require [henacat.util.response_header_generator :refer [ResponseHeaderGenerator]]
            [henacat.util.util :as util]
            [clojure.string :as s]))

(defn get-cookie-date-string
  [cal]
  (let [df (SimpleDateFormat. "EEE, dd-MMM-yyyy HH:mm:ss" Locale/US)]
    (.setTimeZone df (.getTimeZone cal))
    (str (.format df (.getTime cal)) " GMT")))

(defn generate-cookie-strings
  [cookies]
  (map (fn [cookie]
         (s/join "; "
                 (filter #(not (nil? %))
                         (vector
                          (str "Set-Cookie: " (.get-name cookie) "=" (.get-value cookie))
                          (when-not (nil? (.get-domain cookie))
                            (str "Domain=" (.get-domain cookie)))
                          (when (> (.get-max-age cookie) 0)
                            (let [cal (Calendar/getInstance (TimeZone/getTimeZone "UTC"))]
                              (.add cal Calendar/SECOND (.get-max-age cookie))
                              (str "Expires=" (get-cookie-date-string cal))))
                          (when (= (.get-max-age cookie) 0)
                            (let [cal (Calendar/getInstance (TimeZone/getTimeZone "UTC"))]
                              (.set cal 1970 0 1 0 0 10)
                              (str "Expires=" (get-cookie-date-string cal))))
                          (when-not (nil? (.get-path cookie))
                            (str "Path=" (.get-path cookie)))
                          (when (.get-secure cookie)
                            (str "Secure"))
                          (when (.is-http-only cookie)
                            (str "HttpOnly"))))))
       cookies))

(defrecord ResponseHeaderGeneratorImpl [cookies]
  ResponseHeaderGenerator
  (generate [this output]
    (doseq [cookie (generate-cookie-strings (:cookies this))]
      (println cookie)
      (util/write-line output cookie))))


(defn make-ResponseHeaderGeneratorImpl
  [cookies]
  (ResponseHeaderGeneratorImpl. cookies))
