(ns session-test.core
  (:import [javax.servlet.http
            HttpServlet
            HttpSession
            HttpServletRequest
            HttpServletResponse
            ]
           [java.io PrintWriter]
           [java.lang Integer])
  (:gen-class
   :name SessionTest
   :main false
   :extends javax.servlet.http.HttpServlet))

(defn -doGet
  [this ^HttpServletRequest request ^HttpServletResponse response]
  (println this)
  (.setContentType response "text/plain")
  (let [^PrintWriter out (.getWriter response)
        ^HttpSession session (.getSession request true)
        ^Integer counter (Integer. (.getAttribute session "Counter"))]
    (if (nil? counter)
      (do
        (.println out "No Session!!")
        (.setAttribute session "Counter", (Integer. 1)))
      (do
        (.println out (str "Counter.. " (inc (Integer. counter))))))))
