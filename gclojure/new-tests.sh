#!/bin/bash

( cd .. &&
  make OPTIONAL=0 clojure_MODE=cljs test^clojure^step1 )
