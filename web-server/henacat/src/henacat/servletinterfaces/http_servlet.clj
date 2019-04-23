(ns henacat.servletinterfaces.http_servlet)

(defprotocol HttpServlet
  (do-get [this req res])
  (do-post [this req res])
  (service [this req res]))
