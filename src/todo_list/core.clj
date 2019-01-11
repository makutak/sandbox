(ns todo-list.core
  (:require [ring.adapter.jetty :as jetty]
            [ring.middleware.reload :refer [wrap-reload]]
            [compojure.core :refer [defroutes GET]]
            [compojure.route :refer [not-found]]))

(defn welcome
  "A ring handler to process all requests for the web server.
  If a request is for something other than then an error message is returned"
  [request]
  {:status 200
   :body (str "<h1>Hello, Clojure World</h1>"
              "<p>Welcome to your first Clojure app, I now update automically</p>"
              "<p>I now use defroutes to manage incoming requests</p>")
   :headers {}})

(defroutes app
  (GET "/" [] welcome)
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
