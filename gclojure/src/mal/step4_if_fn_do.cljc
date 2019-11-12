(ns mal.step4-if-fn-do
  (:require [mal.readline :as readline]
            [mal.reader :as reader]
            [mal.printer :as printer]
            [mal.env :as mal-env]
            [clojure.repl])
  (:gen-class))

(use 'clojure.tools.trace)

;; read
(defn READ [strng]
  (reader/read-str strng))

;; eval
(declare eval-ast)
(declare EVAL)

(defn eval-def! [args env]
  (if (< 2 (count args)) (throw (Error. (str "def! requires 2 arguments")))
      (do (mal-env/env-set env (first args) (EVAL (second args) env))
          (mal-env/env-get env (first args)))))

(defn eval-let* [args env]
  (if (< 2 (count args)) (throw (Error. (str "let* requires 2 arguments")))
      (let [new-env (mal-env/env-new env)]
        (doall (map (fn [key-value] key-value (mal-env/env-set
                                               new-env (first key-value)
                                               (EVAL (second key-value) new-env)))
                    (partition 2 (first args))))
        (EVAL (second args) new-env))))

(defn eval-if [args env]
  (if (EVAL (first args) env)
    (EVAL (second args) env)
    (if (> (count args) 2)
      (EVAL (nth args 2) env)
      nil)))

(defn EVAL [ast env]
  (cond (and (list? ast) (empty? ast)) ast
        (list? ast) (cond (= (first ast) 'def!) (eval-def! (rest ast) env)
                          (= (first ast) 'let*) (eval-let* (rest ast) env)
                          (= (first ast) 'if) (eval-if (rest ast) env)
                          :else (let [ast-evaled (eval-ast ast env)]
                                  (apply (first ast-evaled) (rest ast-evaled))))
        :else (eval-ast ast env)))

(defn eval-symbol [symbol env]
  (mal-env/env-get env symbol))

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
(defn repl-loop [env]
  (let [line (readline/readline "user> ")]
    (when line
      (when-not (re-seq #"^\s*$|^\s*;.*$" line)             ; blank/comment
        (try
          (println (rep line env))
          (catch Throwable e (clojure.repl/pst e))))
      (recur env))))

(defn -main [& args]
  (let [env (mal-env/env-new nil)]
    (mal-env/env-set env '+ +)
    (mal-env/env-set env '- -)
    (mal-env/env-set env '* *)
    (mal-env/env-set env '/ /)
    (repl-loop env)))
