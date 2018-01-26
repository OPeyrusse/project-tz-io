(ns clj-parser.parser
  (:require [instaparse.core :as insta]))

(def parse
  (insta/parser
    "S = AB*
     AB = A B
     A = #'a*'
     B = #'b*'"))