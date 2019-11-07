(ns mal.reader)
(use 'clojure.test)

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")
(def integer-regex #"^-?(0|[123456789][0123456789]*)$")
(def string-regex #"^\".*\"$")

(defn tokenize [input]
  (map
    (fn [x] (nth x 1))
    (re-seq token-regex input)))
(deftest tokenize-test
  (is (= '("(" "abc" ")" "") (tokenize "(abc)"))))

(defn read-string-token [token]
   (subs token 1 (- (count token) 1)))
(deftest read-string-token-test
  (is (= "abc" (read-string-token "\"abc\""))))

(defn read-atom [token]
  (cond (re-matches integer-regex token) (Integer/parseInt token)
        (re-matches string-regex token) (read-string-token token)
        :else (symbol token)))
(deftest read-atom-test
  (is (= 1 (read-atom "1")))
  (is (= 'symbol (read-atom "symbol")))
  (is (= "abc" (read-atom "\"abc\""))))

(declare read-sequence)

(defn read-form [tokens]
  (let [first-token (first tokens)]
    (cond (= first-token "(") (read-sequence (rest tokens) ")")
          (= first-token "[") (let [vec-seq (read-sequence (rest tokens) "]")]
                                {:value (vec (:value vec-seq)) :tokens (:tokens vec-seq)})
          :else {:value  (read-atom (first tokens))
                 :tokens (rest tokens)})))
(deftest read-form-test
  (is (= {:value 1 :tokens []} (read-form ["1"])))
  (is (= {:value '(1 2 3) :tokens []} (read-form ["(" "1" "2" "3" ")"])))
  (is (= {:value [1 2 3] :tokens []} (read-form ["[" "1" "2" "3" "]"]))))

(defn read-sequence [tokens closing-token]
  (cond (= (count tokens) 0) {:value  '("unbalanced list error")
                              :tokens []}
        (= (first tokens) closing-token) {:value  '()
                                :tokens (drop 1 tokens)}
        :else (let [read-form-output (read-form tokens)
                    form-value (read-form-output :value)
                    rest-tokens (read-form-output :tokens)
                    read-list-output (read-sequence rest-tokens closing-token)
                    rest-of-list (read-list-output :value)
                    rest-tokens (read-list-output :tokens)]
                {:value  (cons form-value
                               rest-of-list)
                 :tokens rest-tokens})))
(deftest read-sequence-test
  (is (= {:value '(1 2 3) :tokens []} (read-sequence ["1" "2" "3" ")"] ")"))))


(defn read-str [strng]
  (let [form (read-form (tokenize strng))]
    (form :value)))
(deftest read-str-test
  (is (= '(1 2 3)) (read-str "(1 2 3)")))
