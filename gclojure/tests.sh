#!/bin/bash

( cd .. &&
  make OPTIONAL=0 GCLOJURE_mode=clj test^gclojure^step0 
  make OPTIONAL=0 GCLOJURE_mode=clj test^gclojure^step1
  make OPTIONAL=0 GCLOJURE_mode=clj test^gclojure^step2)
