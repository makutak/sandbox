(ns henacat.servletinterfaces.http_servlet_response)

(defprotocol HttpServletResponse
  (set-character-encoding [this charset])
  (set-content-type [this content-type])
  (get-writer [this])
  (add-cookie [this cookie]))
