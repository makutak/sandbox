(ns test-bbs.core
  (:import [javax.servlet.http.HttpServlet])
  (:gen-class
   :main false
   :extends javax.servlet.http.HttpServlet))

(defn -init
  [this request response]
  [this]
  (println "init"))

(defn -doGet [this request response]
  (.setContentType response "text/html; charset=UTF-8")
  (let [out (.getWriter response)]
    (.. out
        (println "<html><body>hello world</body></html>"))))

(defn -destroy
  [this]
  (println "destory"))
