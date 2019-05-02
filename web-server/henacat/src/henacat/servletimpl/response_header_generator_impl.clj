(ns henacat.servletimpl.response_header_generator_impl
  (:require [henacat.util.response_header_generator :refer [ResponseHeaderGenerator]]))


(defrecord ResponseHeaderGeneratorImpl [cookies]
  ResponseHeaderGenerator
  (generate [this output]
    ))


(defn make-ResponseHeaderGeneratorImpl
  [cookies]
  (ResponseHeaderGeneratorImpl. cookies))
