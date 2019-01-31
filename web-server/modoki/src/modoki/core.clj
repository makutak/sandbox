(ns modoki.core
  (:require [modoki.server :refer [server]])
  (:gen-class))

(defn -main
  []
  (server (Integer. 8001)))
