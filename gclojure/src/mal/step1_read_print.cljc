(ns mal.step1-read-print
  (:require [mal.readline :as readline]
            [mal.reader :as reader])
  #?(:clj (:gen-class)))

;; read
(defn READ [& [strng]]
  (reader/tokenize strng))

;; eval
(defn EVAL [ast env]
  ast)

;; print
(defn PRINT [exp]
  exp)

;; repl
(defn rep [strng] (PRINT (EVAL (READ strng), {})))
;; repl loop
(defn repl-loop []
  (let [line (readline/readline "user> ")]
    (when line
      (when-not (re-seq #"^\s*$|^\s*;.*$" line) ; blank/comment
        (println (rep line)))
      (recur))))

(defn -main [& args]
  (repl-loop))
