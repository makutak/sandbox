(defproject clojurescript-tdd-application "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.10.1"]
                 [org.clojure/clojurescript "1.10.764"]]
  :plugins [[lein-cljsbuild "1.1.8" :exclusions [[org.clojure/clojure]]]
            [lein-doo "0.1.10"]
            [lein-figwheel "0.5.20"]]
  :clean-targets ^{:protect false} [:target-path "out" "resources/public/cljs"]
  :figwheel {:css-dirs ["resources/public/css"]}
  :cljsbuild {
              :builds [{:id "dev"       ; development configuration
                        :source-paths ["src"] ; Paths to monitor for build
                        :figwheel true        ; Enable Figwheel
                        :compiler {:main clojurescript_tdd_application.core ; your main namespace
                                   :asset-path "cljs/out" ; Where load-dependent files will go, mind you this one is relative
                                   :output-to "resources/public/cljs/main.js" ; Where the main file will be built
                                   :output-dir "resources/public/cljs/out" ; Directory for temporary files
                                   :source-map-timestamp true} ; Sourcemaps hurray!
                        }
                       {:id "test"
                        :source-paths ["src" "test"]
                        :compiler {:main runners.doo
                                   :optimizations :none
                                   :output-to "resources/public/cljs/tests/all-tests.js"}}]})
