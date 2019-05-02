(ns henacat.util.response_header_generator)

(defprotocol ResponseHeaderGenerator
  (generator [this]))
