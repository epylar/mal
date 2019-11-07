(ns mal.reader)
(use 'clojure.test)
(require '[clojure.edn :as edn])

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")

(defn tokenize [input]
  (map
    (fn [x] (nth x 1))
    (re-seq token-regex input)))
(deftest tokenize-test
  (is (= '("(" "abc" ")" "") (tokenize "(abc)"))))

(defn read-atom [token]
  (edn/read-string token))
(deftest read-atom-test
  (is (= 1 (read-atom "1"))))

(declare read-list)

(defn read-form [tokens]
  (let [first-token (first tokens)]
    (if (= first-token "(") (read-list (rest tokens))
                            (list (read-atom (first tokens)) (rest tokens)))))
(deftest read-form-test
  (is (= '(1 [])) (read-form ["1"]))
  (is (= '((1 2 3) []) (read-form ["(" "1" "2" "3" ")"]))))

(defn read-list [tokens]
  (let [next-symbol (first tokens)]
    (if (= next-symbol ")")
      (list '() (drop 1 tokens))
      (let [read-form-output (read-form  tokens)
            next-form (first read-form-output)
            remaining-tokens-after-form (nth read-form-output 1)
            read-list-output (read-list remaining-tokens-after-form)
            rest-of-list (first read-list-output)
            remaining-tokens-after-list (nth read-list-output 1)]
        (list (cons next-form rest-of-list) remaining-tokens-after-list)))))
(deftest read-list-test
  (is (= '((1 2 3) [])) (read-list ["1" "2" "3" ")"])))



(defn read-str [strng]
  (let [form (read-form (tokenize strng))]
    (first form)))
(deftest read-str-test
  (is (= '(1 2 3)) (read-str "(1 2 3)")))
