(ns henacat.servletinterfaces.http_servlet
  (:gen-class
   :name henacat.servletinterfaces.HttpServlet
   :main false
   :methods [[doService [] void]
             [doPost [] void]
             [doGet [] void]]))

(defn -doService
  [this]
  (println this))

(defn -doGet
  [this]
  (println this))

(defn -doPost
  [this]
  (println this))
