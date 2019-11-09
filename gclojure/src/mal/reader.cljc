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

(defn reader [tokens]
  {:tokens  tokens :position (atom 0)})

(defn peek-next [reader]
  (if (< @(:position reader) (count (:tokens reader)))
    (nth (:tokens reader) @(:position reader) )
    nil))

(defn read-next [reader]
  (let [next-val (peek-next reader)]
    (if (not (= next-val nil))
      (do (swap! (:position reader) (fn [x] (+ x 1)))
          next-val)
      nil)))

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

(defn read-keyword [token]
  (keyword (subs token 1)))

(defn read-form [reader]
  (let [first-token (read-next reader)]

    (cond (= first-token "(") (read-sequence reader ")")
          (= first-token "[") (vec (read-sequence reader "]"))
          (= (first (seq first-token)) \:) (read-keyword first-token)
          :else   (read-atom  first-token))))

(deftest read-form-test
  (is (= 1 (read-form (reader ["1"]))))
  (is (=  '(1 2 3) (read-form (reader ["(" "1" "2" "3" ")"]))))
  (is (=   :foo (read-form (reader [":foo"]))))
  (is (=  [1 2 3] (read-form (reader ["[" "1" "2" "3" "]"])))))

(defn read-sequence [reader closing-token]
  (cond (= (peek-next reader) nil)   '("unbalanced list error")
        (= (peek-next reader) closing-token) '()
        :else (let [
                    form-value (read-form reader)
                    rest-of-list (read-sequence reader closing-token)]
        (cons form-value rest-of-list))))

(deftest read-sequence-test
  (is (= '(1 2 3) (read-sequence (reader ["1" "2" "3" ")"]) ")"))))

(defn read-str [strng]
  (read-form (reader (tokenize strng))))
(deftest read-str-test
  (is (= '(1 2 3) (read-str "(1 2 3)"))))
