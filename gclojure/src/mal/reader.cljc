(ns mal.reader)
(use 'clojure.test)

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")

(defn tokenize [input] (map (fn [x] (nth x 1)) (re-seq token-regex input)))
(deftest tokenize
  (is (= '("(" "a" "b" "c" ")") (tokenize "(abc)"))))

;;
(defn read-form [input] )