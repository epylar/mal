#!/bin/bash

( cd .. &&
  make REGRESS=1 OPTIONAL=0 GCLOJURE_mode=cljs test^gclojure^step3 )
