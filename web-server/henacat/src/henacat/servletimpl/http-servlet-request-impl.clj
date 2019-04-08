(ns henacat.servletimpl.http-servlet-request-impl
  (:import [javax.servlet.http.HttpServlet])
  (:gen-class
   :name henacat.servletimpl.http-servlet-request-impl
   :main false
   :implements [javax.servlet.http.HttpServlet]))

(defn -getMethod
  [this]
  (.method this))
