(ns henacat.test_BBS
  (:import [javax.servlet.http HttpServlet HttpServletRequest HttpServletResponse])
  (:gen-class
   :name henacat.TestBBS
   :extends javax.servlet.http.HttpServlet))

(defn test-BBS
  []
  (println "TestBBS!"))
