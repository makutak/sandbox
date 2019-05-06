(ns henacat.servletimpl.servlet_service
  (:import [java.nio.file FileSystems]
           [java.net URLClassLoader URL]
           [java.util HashMap]
           [java.lang StringBuilder AssertionError]
           [java.io ByteArrayOutputStream])
  (:require [clojure.java.io :refer [as-url]]
            [clojure.string :as s]
            ;; [henacat.servletinterfaces.http_servlet :refer [HttpServlet]]
            [henacat.servletimpl.http_servlet_request_impl :refer [make-HttpServletRequestImpl]]
            [henacat.servletimpl.http_servlet_response_impl :refer [make-HttpServletResponseImpl]]
            [henacat.servletimpl.response_header_generator_impl :refer [make-ResponseHeaderGeneratorImpl]]
            [henacat.util.send_response :refer [send-ok-response-header]]))

(defn create-servlet
  [info]
  (let [fs (FileSystems/getDefault)
        path-obj (.getPath fs (:servlet-directory info) (into-array [""]))
        url (as-url (.toUri path-obj))
        loader (URLClassLoader/newInstance (into-array [url]))
        clazz (.loadClass loader (:servlet-className info))]
    (.newInstance clazz)))

(defn string->map
  [string]
  (let [parameter-map {}]
    (if (not (empty? string))
      (reduce
       (fn [param-map param]
         (let [key-value (s/split param #"=")]
           (assoc param-map (keyword (first key-value)) (last key-value))))
       parameter-map
       (s/split  string #"\&"))
      parameter-map)))

(defn read->size
  [input size]
  (loop [ch (.read input)
         sb (StringBuilder.)
         read-size 0]
    (if (and (< read-size size)
             (not= ch -1)
             (not= 0 (.available input)))
      (do
        (recur
         (.read input)
         (.append sb (char ch))
         (inc read-size)))
      (do
        (.append sb (char ch))
        (.toString sb)))))

(defn do-service
  [method query info request-header input output]
  ;; (:servlet info) が nilなら create-servletする
  (when (nil? @(:servlet info))
    (reset! (:servlet info) (create-servlet info)))
  (let [output-buffer (ByteArrayOutputStream.)]
    (cond
      ;; methodがGETのとき
      (= method "GET")
      (let [param-map (string->map query)
            req (make-HttpServletRequestImpl "GET" request-header param-map)
            resp (make-HttpServletResponseImpl output-buffer)
            hg (make-ResponseHeaderGeneratorImpl (:cookies resp))]
        (.service @(:servlet info) req resp)
        (send-ok-response-header output @(:content-type resp) hg)
        (.flush @(:print-writer resp))
        (doseq [b (.toByteArray output-buffer)]
          (.write output (int b)))
        (.flush output))

      ;; methodがPOSTのとき
      (= method "POST")
      (let [content-length (Integer. (:CONTENT-LENGTH request-header))
            line (read->size input content-length)
            param-map (string->map line)
            req (make-HttpServletRequestImpl "POST" request-header param-map)
            resp (make-HttpServletResponseImpl output-buffer)
            hg (make-ResponseHeaderGeneratorImpl (:cookies resp))]
        (.service @(:servlet info) req resp)
        (send-ok-response-header output @(:content-type resp) hg)
        (.flush @(:print-writer resp))
        (doseq [b (.toByteArray output-buffer)]
          (.write output (int b)))
        (.flush output))

      ;; methodがそれ以外のとき
      :else (AssertionError. (str "BAD METHOD:" method)))))
