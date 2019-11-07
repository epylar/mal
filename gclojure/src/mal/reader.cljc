(ns mal.reader)
(use 'clojure.test)

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")
(def integer-regex #"-?(0|[123456789][0123456789]*)")

(defn tokenize [input]
  (map
    (fn [x] (nth x 1))
    (re-seq token-regex input)))
(deftest tokenize-test
  (is (= '("(" "abc" ")" "") (tokenize "(abc)"))))

(defn read-atom [token]
  (if (re-matches integer-regex token)
    (Integer/parseInt token)
    (symbol token)))
(deftest read-atom-test
  (is (= 1 (read-atom "1")))
  (is (= 'symbol (read-atom "symbol"))))

(declare read-list)

(defn read-form [tokens]
  (let [first-token (first tokens)]
    (if (= first-token "(") (read-list (rest tokens))
                            {:value (read-atom (first tokens))
                             :tokens (rest tokens)})))
(deftest read-form-test
  (is (= {:value 1 :tokens []} (read-form ["1"])))
  (is (= {:value '(1 2 3) :tokens []} (read-form ["(" "1" "2" "3" ")"]))))

(defn read-list [tokens]
  (cond (= (count tokens) 0) {:value  '("unbalanced list error")
                              :tokens []}
        (= (first tokens) ")") {:value  '()
                                :tokens (drop 1 tokens)}
        :else (let [read-form-output (read-form tokens)
                    form-value (read-form-output :value)
                    rest-tokens (read-form-output :tokens)
                    read-list-output (read-list rest-tokens)
                    rest-of-list (read-list-output :value)
                    rest-tokens (read-list-output :tokens)]
                {:value  (cons form-value
                               rest-of-list)
                 :tokens rest-tokens})))
(deftest read-list-test
  (is (= {:value '(1 2 3) :tokens []} (read-list ["1" "2" "3" ")"]))))


(defn read-str [strng]
  (let [form (read-form (tokenize strng))]
    (form :value)))
(deftest read-str-test
  (is (= '(1 2 3)) (read-str "(1 2 3)")))
