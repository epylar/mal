(ns mal.printer)
(use 'clojure.test)

(defn mal-pr-str [form]
  (pr-str form))
(deftest mal-pr-str-test
  (is (= "(1 2 3)" (pr-str '(1 2 3))))
  (is (= ":symbol" (pr-str :symbol))))