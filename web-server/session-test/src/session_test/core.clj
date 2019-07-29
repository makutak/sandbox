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
  [^HttpServlet this ^HttpServletRequest request ^HttpServletResponse response]
  (.setContentType response "text/plain")
  (let [^PrintWriter out (.getWriter response)
        ^HttpSession session (.getSession request true)
        counter (.getAttribute session "Counter")]
    (if (nil? counter)
      (do
        (.println out "No Session!!")
        (.setAttribute session "Counter" 1))
      (do
        (.println out (str "Counter.. " counter))
        (.setAttribute session "Counter" (inc (Integer. counter)))))))
