(ns test-bbs.core
  (:import [javax.servlet.http.HttpServlet])
  (:gen-class
   :name TestBBS
   :main false
   :extends javax.servlet.http.HttpServlet))

(defn -doGet [this request response]
  (.setContentType response "text/html; charset=UTF-8")
  (let [out (.getWriter response)]
    (.println out "<html>")
    (.println out "<head>")
    (.println out "<title>テスト掲示板</title>")
    (.println out "</head>")
    (.println out "<body>")
    (.println out "<h1>テスト掲示板</h1>")
    (.println out "<form action='/test-bbs/TestBBS' method=POST>")
    (.println out "ハンドル名: <input type='text' name='handle'><br/>")
    (.println out "<textarea name='message' rows='4' cols='60'></textarea><br/>")
    (.println out "<input type='submit' />")
    (.println out "</form>")
    (.println out "<hr />")
    (.println out "</body>")
    (.println out "</html>")))
