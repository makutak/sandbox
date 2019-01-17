(ns todo-list.core
  (:require [ring.adapter.jetty :as jetty]
            [ring.middleware.reload :refer [wrap-reload]]
            [compojure.core :refer [defroutes GET]]
            [compojure.route :refer [not-found]]
            [ring.handler.dump :refer [handle-dump]]
            [hiccup.core :refer :all]
            [hiccup.page :refer :all]))

(defn welcome
  "A ring handler to process all requests for the web server.
  If a request is for something other than then an error message is returned"
  [request]
  (html [:h1 "Hello, Clojure World"]
        [:p "Welcome to your first Clojure app, I now update automically"]))

(defn goodbye
  [request]
  (html5 {:lang "en"}
         [:head (include-js "myscript.js") (include-css "mystyle.css")]
         [:body
          [:div [:h1  {:class "info"} "Walking back to happiness"]]
          [:div [:p "Walking back to happiness with you"]]
          [:div [:p "Said, Farewell to loneliness I knew"]]
          [:div [:p "Laid aside foolish pride"]]
          [:div [:p "Learnt the truth from tears I cried"]]]))

(defn about
  "Information about the webpage developer"
  [request]
  {:status 200
   :body (str "I am an awesome clojure developer, well getting there...")
   :headers {}})

(defn hello
  "A simple personalised greeting showing the use of variable path elements"
  [request]
  (let [name (get-in request [:route-params :name])]
    {:status 200
     :body (str "Hello " name ". I got your name from the web URL")
     :headers {}}))

(def operands {"+" +
               "-" -
               "*" *
               ":" /})

(defn calculator
  "A very simple calculator that can add, divide, subtract and multiply.
  This is done through the magic of variable path elements."
  [request]
  (let [op (get-in request [:route-params :op])
        a (Integer. (get-in request [:route-params :a]))
        b (Integer. (get-in request [:route-params :b]))
        f (get operands op)]
    (if f
      {:status 200
       :body (str "calculate resulst is " (f a b))
       :headers {}}
      {:status 404
       :body "Sorry, unknown operator.  I only recognise + - * : (: is for division)"
       :headers {}})))

(defn hiccup-test
  [request]
  {:status 200
   :body (hiccup.core/html [:div
                            [:ul
                             (for [x (range 1 4)]
                               [:li x])]])
   :headers {}})

(defroutes app
  (GET "/" [] welcome)
  (GET "/goodbye" [] goodbye)
  (GET "/about" [] about)
  (GET "/request-info" [] handle-dump)
  (GET "/hello/:name" [] hello)
  (GET "/calculator/:op/:a/:b" [] calculator)
  (GET "/hiccup-test" [] hiccup-test)
  (not-found (str "<h1>This is not the page you are looking for</h1>"
                  "<p>Sorry, the page you requested was not found!</p>")))

(defn -main
  "A very simple web server using Ring & Jetty"
  [port-number]
  (jetty/run-jetty app
                   {:port (Integer. port-number)}))

(defn -dev-main
  "A very simple web server using Ring & Jetty
  that reloads code changes via the development profile of Leiningen"
  [port-number]
  (jetty/run-jetty (wrap-reload #'app)
                   {:port (Integer. port-number)}))
