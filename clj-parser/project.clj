(defproject clj-parser "0.1.0-SNAPSHOT"
  :description "Clojure parser for TZ I/O language"
  :url "https://github.com/Kineolyan/project-tz-io"
  :license {:name "MIT"
            :url "https://github.com/Kineolyan/project-tz-io/blob/master/LICENSE"}
  :dependencies
    [
      [org.clojure/clojure "1.8.0"]
      [instaparse "1.4.8"]]
  :main ^:skip-aot clj-parser.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
