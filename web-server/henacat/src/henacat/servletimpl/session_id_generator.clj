(ns henacat.servletimpl.session_id_generator
  (:import [java.security SecureRandom]))


(def generate-session-id
  []
  )

(defn bytes
  "Returns a random byte array of the specified size."
  [size]
  (let [seed (byte-array size)]
    (.nextBytes (SecureRandom.) seed)
    seed))
