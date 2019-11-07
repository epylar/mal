(ns mal.reader)

(def token-regex #"[\s,]*(~@|[\[\]{}()'`~^@]|\"(?:\\.|[^\\\"])*\"?|;.*|[^\s\[\]{}('\"`,;)]*)")

(defn tokenize [input] (map (fn [x] (nth x 1)) (re-seq token-regex input)))
