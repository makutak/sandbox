(ns henacat.servletimpl.http_servlet_response_impl)

(def default-content-type "application/octet-stream")
(def default-character-encoding "UTF-8")

(defrecord HttpServletResponseImpl [content-type character-encoding output-stream print-writer])
