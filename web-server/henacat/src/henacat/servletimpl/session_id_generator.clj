(ns henacat.servletimpl.session_id_generator
  (:import [java.security SecureRandom])
  (:require [clojure.string :as s]))


(defn generate-session-id
  []
  (let [random (SecureRandom/getInstance "SHA1PRNG")
        bytes (byte-array 16)]
    (.nextBytes random bytes)
    (s/join ""
            (map (fn [b] (->> (bit-and b 0xff)
                              (Integer/toHexString)
                              (s/upper-case)))
                 bytes))))
