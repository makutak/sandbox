(ns henacat.servletimpl.servlet_service
  (:import [java.nio.file FileSystems]
           [java.net URLClassLoader URL]
           [java.util HashMap]
           [java.lang StringBuilder AssertionError])
  (:require [clojure.java.io :refer [as-url]]
            [clojure.string :as s]
            ;; [henacat.servletinterfaces.http_servlet :refer [HttpServlet]]
            [henacat.servletimpl.http_servlet_request_impl :refer [make-HttpServletRequestImpl]]
            [henacat.servletimpl.http_servlet_response_impl :refer [make-HttpServletResponseImpl]]))

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
         read-size 1]
    (if (and (< read-size size)
             (not= ch -1))
      (do
        (recur
         (.read input)
         (.append sb (char ch))
         (inc read-size)))
      (do
        (.toString sb)))))

(defn do-service
  [method query info request-header input output]
  ;; (:servlet info) が nilなら create-servletする
  (when (= (nil? (:servlet info)))
    (reset! (:servlet info) (create-servlet info)))

  (cond
    ;; methodがGETのとき
    ;;;; map = stringToMap(query);
    ;;;; req = new HttpServletRequestImpl("GET", map);
    (= method "GET")
    (let [param-map (string->map query)
          req (make-HttpServletRequestImpl "GET" param-map)
          resp (make-HttpServletResponseImpl output)]
      (.doGet @(:servlet info) req resp)
      (.flush @(:print-writer resp)))

    ;; methodがPOSTのとき
    ;;;; Content-Lengthを取得
    ;;;; String line = readToSize(input, contentLength);
    ;;;; req = new HttpServletRequestImpl("POST", map);
    (= method "POST")
    (let [content-length (Integer. (:CONTENT-LENGTH request-header))
          line (read->size input content-length)
          param-map (string->map line)
          req (make-HttpServletRequestImpl "POST" param-map)
          resp (make-HttpServletResponseImpl output)]
      (.doPost @(:servlet info) req resp)
      (.flush @(:print-writer resp)))

    :else (AssertionError. (str "BAD METHOD:" method)))


  ;; HttpServletResponseImpl resp = new HttpServletResponseImpl(output);
  ;; info.servlet.service(req, resp);
  ;; resp.printWriter.flush();
  )
