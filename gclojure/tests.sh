#!/bin/bash

( cd .. &&
  make OPTIONAL=0 gclojure_MODE=clj test^gclojure^step0 
  make OPTIONAL=0 gclojure_MODE=clj test^gclojure^step1
  make OPTIONAL=0 gclojure_MODE=clj test^gclojure^step2
  make REGRESS=1 OPTIONAL=0 gclojure_MODE=clj test^gclojure^step3)
