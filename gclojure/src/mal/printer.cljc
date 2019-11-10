(ns mal.printer)
(use 'clojure.test)
(use 'clojure.tools.trace)

(declare mal-pr-str)

(defn pr-map [the-map]
  (str "{"
       (clojure.string/join " " (map
                                 (fn [map-key] (str  (mal-pr-str map-key)
                                                     " "
                                                     (mal-pr-str (get the-map map-key))))
                                 (keys the-map)))
       "}"))

(defn pr-list [the-list]
  (str "("
       (clojure.string/join " " (map mal-pr-str the-list))
       ")"))

(defn pr-vector [the-vector]
  (str "["
       (clojure.string/join " " (map mal-pr-str the-vector))
       "]"))

(defn mal-pr-str [form]
  (cond (list? form) (pr-list form)
        (vector? form) (pr-vector form)
        (map? form) (pr-map form)
        :else (pr-str form)))
(deftest mal-pr-str-test
  (is (= "(1 2 3)" (pr-str '(1 2 3))))
  (is (= ":symbol" (pr-str :symbol))))

