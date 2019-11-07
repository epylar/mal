(ns mal.printer)
(use 'clojure.test)

(defn mal-pr-str [form]
  (str form))
(deftest mal-pr-str-test
  (is (= "(1 2 3)" (pr-str '(1 2 3)))))