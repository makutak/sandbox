(ns henacat.servletimpl.http_servlet_response_impl)

(def default-content-type "application/octet-stream")
(def default-character-encoding "UTF-8")

(defprotocol IHttpServletResponseImpl)

(defrecord HttpServletResponseImpl [content-type character-encoding output-stream print-writer])
