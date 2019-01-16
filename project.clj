(defproject todo-list "0.1.0-SNAPSHOT"
  :description "A simple webapp using Ring"
  :url "http://example.com/FIXME"
  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}
  :dependencies [[org.clojure/clojure "1.9.0"]
                 [ring "1.7.1"]
                 [compojure "1.6.1"]
                 [hiccup "2.0.0-alpha1"]]
  :repl-options {:init-ns todo-list.core}
  :main todo-list.core
  :min-lein-version "2.0.0"
  :uberjar-name: "todo-list.jar"
  :profiles {:dev
             {:main todo-list.core/-dev-main}})
