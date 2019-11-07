(ns mal.printer)
(use 'clojure.test)

(defn pr-str [form]
  (str form))
(deftest pr-str-test
  (is (= "(1 2 3)" (pr-str '(1 2 3)))))