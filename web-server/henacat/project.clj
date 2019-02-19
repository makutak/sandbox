(defproject henacat "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.9.0"]]
  :main ^:skip-aot henacat.core
  :target-path "target/%s"
  :aot [henacat.test_BBS
        henacat.message]
  :resource-paths ["lib/servlet-api.jar"]
  :profiles
  {:dev {:dependencies [[javax.servlet/javax.servlet-api "3.1.0"]]}
   :uberjar {:aot :all}})
