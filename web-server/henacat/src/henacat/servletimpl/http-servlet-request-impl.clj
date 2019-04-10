(ns henacat.servletimpl.http-servlet-request-impl
  (:import [javax.servlet.http HttpServletRequest])
  (:gen-class
   :name henacat.servletimpl.http-servlet-request-impl
   :main false
   :implements [javax.servlet.http.HttpServletRequest]))

(defn -getMethod
  [this]
  (.method this))

(defn -getParameter
  [this ^String name])

(defn -setCharacterEncoding
  [this ^String env])
