(ns clj-parser.core
  (:gen-class)
  (:require [clj-parser.parser :as p]))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (p/parse "aabba")))
