(ns mal.step2-eval
  (:require [mal.readline :as readline]
            [mal.reader :as reader]
            [mal.printer :as printer]
            [clojure.repl])
  (:gen-class))

(use 'clojure.tools.trace)

;; read
(defn READ [strng]
  (reader/read-str strng))

;; eval
(declare eval-ast)

(defn EVAL [ast env]
  (cond (and (list? ast) (empty? ast)) ast
        (list? ast) (let [ast-evaled (eval-ast ast env)]
                      (apply (first ast-evaled) (rest ast-evaled)))
        :else (eval-ast ast env)))

(defn eval-symbol [symbol env]
  (let [lookup (get env symbol)]
    (if (= lookup nil) (throw (Error. (str symbol " not found"))))
    lookup))

(defn eval-ast [ast env]
  (cond (symbol? ast) (eval-symbol ast env)
        (list? ast) (map (fn [x] (EVAL x env)) ast)
        (vector? ast) (vec (map (fn [x] (EVAL x env)) ast))
        (map? ast) (reduce conj (map (fn [key] {key (EVAL (get ast key) env)}) (keys ast)))
        :else ast))

;; print
(defn PRINT [exp]
  (printer/mal-pr-str exp))

;; repl
(defn rep [strng env] (PRINT (EVAL (READ strng) env)))
;; repl loop
(defn repl-loop []
  (let [line (readline/readline "user> ")
        env {'+ +
             '- -
             '/ /
             '* *}]
    (when line
      (when-not (re-seq #"^\s*$|^\s*;.*$" line) ; blank/comment
        (try
          (println (rep line env))
          (catch Throwable e (clojure.repl/pst e))))
      (recur))))

(defn -main [& args]
  (repl-loop))
