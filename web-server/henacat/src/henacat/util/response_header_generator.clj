(ns henacat.util.response_header_generator)

(defprotocol ResponseHeaderGenerator
  (generate [this]))
