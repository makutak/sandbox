(ns test-bbs.core
  (:import [javax.servlet.http.HttpServlet])
  (:gen-class
   :main false
   :extends javax.servlet.http.HttpServlet))

(defn -servlet
  [this request response]
  (.setContentType response "text/html; charset=UTF-8")
  (let [out (.getWriter response)]
    (.. out
        (println "<html><body>hello world</body></html>"))))

(defn -doGet [this request response]
  (-servlet this request response))
