(ns henacat.test_BBS
  (:import [javax.servlet.http HttpServlet HttpServletRequest HttpServletResponse])
  (:gen-class
   :name henacat.TestBBS
   :extends javax.servlet.http.HttpServlet))

(defn -doGet
  [^HttpServletRequest request ^HttpServletResponse response]
  (.setContentType response "text/html;charset=UTF-8")
  (let [out (.getWriter response)]
    (.println out "<html>")
    (.println out "<head>")
    (.println out "</head>")
    (.println out "<body>")
    (.println out "<h1>hello world</h1>")
    (.println out "</body>")
    (.println out "</html>")))
