(ns henacat.servletimpl.response_header_generator_impl
  (:import [java.text SimpleDateFormat]
           [java.util Date Calendar TimeZone Locale])
  (:require [henacat.util.response_header_generator :refer [ResponseHeaderGenerator]]))

(defn get-cookie-date-string
  [cal]
  (let [df (SimpleDateFormat. "EEE, dd-MMM-yyyy HH:mm:ss" Locale/US)]
    (.setTimeZone df (.getTimeZone cal))
    (str (.format df (.getTime cal)) " GMT")))

(defrecord ResponseHeaderGeneratorImpl [cookies]
  ResponseHeaderGenerator
  (generate [this output]
    ))


(defn make-ResponseHeaderGeneratorImpl
  [cookies]
  (ResponseHeaderGeneratorImpl. cookies))
