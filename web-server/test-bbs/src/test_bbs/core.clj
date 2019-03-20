(ns test-bbs.core
  (:import [javax.servlet.http.HttpServlet]
           [java.util ArrayList])
  (:require [clojure.string :as s])
  (:gen-class
   :name TestBBS
   :main false
   :extends javax.servlet.http.HttpServlet))

(def +message-list+ (ArrayList.))

(defn add-messges
  [message]
  (.add +message-list+ 0 message))

(defn html-escape
  [src]
  (-> src
      (s/replace #"&" "&amp")
      (s/replace #"<" "&lt")
      (s/replace #">" "&gt")))

(defn -doGet
  [this request response]
  (.setContentType response "text/html; charset=UTF-8")
  (let [out (.getWriter response)]
    (.println out "<!DOCTYPE html>")
    (.println out "<html lang='ja'>")
    (.println out "<html>")
    (.println out "<head>")
    (.println out "<meta charset='utf-8'/>")
    (.println out "<title>テスト掲示板</title>")
    (.println out "</head>")
    (.println out "<body>")
    (.println out "<h1>テスト掲示板</h1>")
    (.println out "<form action='/test-bbs/TestBBS' method=POST>")
    (.println out "ハンドル名: <input type='text' name='handle'><br/>")
    (.println out "<textarea name='message' rows='4' cols='60'></textarea><br/>")
    (.println out "<input type='submit' />")
    (.println out "</form>")
    (doseq [message +message-list+]
      (.println out (str "<p>" (html-escape (:handle message)) " さん" "</p>"))
      (.println out "<p>")
      (.println out (s/replace (html-escape (:message message)) "\n" "<br>"))
      (.println out (str "</p>" "<hr />")))
    (.println out "</body>")
    (.println out "</html>")))

(defn parse-params
  [request]
  (let [handle (.getParameter request "handle")
        message(.getParameter request "message")]
    {:handle handle
     :message message}))


(defn -doPost
  [this request response]
  (.setCharacterEncoding request "UTF-8")
  (let [new-message (parse-params request)]
    (add-messges new-message))
  (-doGet this request response))
