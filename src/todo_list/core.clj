(ns todo-list.core
  (:require [ring.adapter.jetty :as jetty]
            [ring.middleware.reload :refer [wrap-reload]]
            [compojure.core :refer [defroutes GET]]
            [compojure.route :refer [not-found]]
            [ring.handler.dump :refer [handle-dump]]))

(defn welcome
  "A ring handler to process all requests for the web server.
  If a request is for something other than then an error message is returned"
  [request]
  {:status 200
   :body (str "<h1>Hello, Clojure World</h1>"
              "<p>Welcome to your first Clojure app, I now update automically</p>"
              "<p>I now use defroutes to manage incoming requests</p>")
   :headers {}})

(defn goodbye
  [request]
  {:status 200
   :body (str "<h1>Walking back to happiness</h1>"
              "<p>Walking back to happiness with you</p>"
              "<p>Said, Farewell to loneliness I knew</p>"
              "<p>Laid aside foolish pride</p>"
              "<p>Learnt the truth from tears I cried</p>")
   :headers {}})

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

(defn calculator
  []
  "")

(defroutes app
  (GET "/" [] welcome)
  (GET "/goodbye" [] goodbye)
  (GET "/about" [] about)
  (GET "/request-info" [] handle-dump)
  (GET "/hello/:name" [] hello)
  (GET "/calculator/:op/:a/:b" [] calculator)
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
