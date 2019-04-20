(ns henacat.servletimpl.servlet_service
  (:import [java.nio.file FileSystems]
           [java.net URLClassLoader URL]
           [javax.servlet.http HttpServlet]
           [java.util HashMap]
           [java.lang StringBuilder AssertionError])
  (:require [clojure.java.io :refer [as-url]]
            [clojure.string :as s]
            [henacat.servletimpl.http_servlet_request_impl :refer [make-HttpServletRequestImpl]]
            [henacat.servletimpl.http_servlet_response_impl :refer [make-HttpServletResponseImpl]]))

(defn create-servlet
  [info]
  (let [fs (FileSystems/getDefault)
        path-obj (.getPath fs (:servlet-directory info) (into-array [""]))
        url (as-url (.toUri path-obj))
        loader (URLClassLoader/newInstance (into-array [url]))
        clazz (.loadClass loader (:servlet-className info))]
    (cast HttpServlet (.newInstance clazz))))

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
    (when (and (< read-size size)
               (not (= -1 ch)))
      (.append sb (char ch))
      (inc read-size))
    (.toString sb)))

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
      (.service @(:servlet info) req resp)
      (.flush (:print-wrter resp)))

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
      (.service @(:servlet info) req resp)
      (.flush (:print-wrter resp)))

    :else (AssertionError. (str "BAD METHOD:" method)))


  ;; HttpServletResponseImpl resp = new HttpServletResponseImpl(output);
  ;; info.servlet.service(req, resp);
  ;; resp.printWriter.flush();
  )