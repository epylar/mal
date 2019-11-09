(ns mal.reader)
(use 'clojure.test)
(use 'clojure.tools.trace)

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")
(def integer-regex #"^-?(0|[123456789][0123456789]*)$")
(def good-string-regex #"^\"(?:\\.|[^\\\"])*\"$")
(def bad-string-regex #"^\"(?:\\.|[^\\\"])*$")

(defn tokenize [input]
  (vec (map
         (fn [x] (nth x 1))
         (re-seq token-regex input))))
(deftest tokenize-test
  (is (= '("(" "abc" ")" "") (tokenize "(abc)"))))

(defn reader [tokens]
  {:tokens  tokens :position (atom 0)})

(defn peek-next [reader]
  (get (:tokens reader) @(:position reader)))

(defn read-next [reader]
  (get (:tokens reader) (dec (swap! (:position reader) (fn [x] (+ x 1))))))

(deftest reader-test
   (let [reader (reader [1 2 3])]
     (is (= 1 (peek-next reader)))
     (is (= 1 (read-next reader)))
     (is (= 2 (read-next reader)))
     (is (= 3 (read-next reader)))
     (is (= nil (read-next reader)))))


(defn read-string-token [token]
  (subs token 1 (- (count token) 1)))
(deftest read-string-token-test
  (is (= "abc" (read-string-token "\"abc\""))))

(defn read-atom [token]
  (cond (nil? token) "ERROR: reading atom from nil"
        (re-matches integer-regex token) (Integer/parseInt token)
        (re-matches good-string-regex token) (read-string-token token)
        (re-matches bad-string-regex token) "ERROR: unbalanced string"
        :else (symbol token)))
(deftest read-atom-test
  (is (= 1 (read-atom "1")))
  (is (= 'symbol (read-atom "symbol")))
  (is (= "abc" (read-atom "\"abc\""))))

(declare read-sequence)

(defn read-keyword [token]
  (keyword (subs token 1)))

(defn hash-from-sequence [sequence]
  (let [k-v-pairs (partition 2 sequence)
        hash-list (map (fn [k-v-pair]
                         {(get (vec k-v-pair) 0)
                          (get (vec k-v-pair) 1)})
                       k-v-pairs)]
    (apply conj hash-list)))

(defn read-form [reader]
  (let [next-token (read-next reader)]
    (cond (= next-token "'") (list 'quote (read-form reader))
          (= next-token "`") (list 'quasiquote (read-form reader))
          (= next-token "~") (list 'unquote (read-form reader))
          (= next-token "~@") (list 'splice-unquote (read-form reader))
          (= next-token "(") (read-sequence reader ")")
          (= next-token "[") (vec (read-sequence reader "]"))
          (= next-token "{") (hash-from-sequence (read-sequence reader "}"))
          (= (first (seq next-token)) \:) (read-keyword next-token)
          :else   (read-atom  next-token))))

(deftest read-form-test
  (is (= 1 (read-form (reader ["1"]))))
  (is (=  '(1 2 3) (read-form (reader ["(" "1" "2" "3" ")"]))))
  (is (=   :foo (read-form (reader [":foo"]))))
  (is (=  [1 2 3] (read-form (reader ["[" "1" "2" "3" "]"]))))
  (is (= '(quote 1) (read-form (reader ["'", "1", ""])))))

(defn read-sequence [reader closing-token]
  (cond (= (peek-next reader) nil)   '("unbalanced list error")
        (= (peek-next reader) closing-token) (do (read-next reader) '())
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
