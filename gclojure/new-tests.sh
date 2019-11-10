#!/bin/bash

( cd .. &&
  make OPTIONAL=0 GCLOJURE_mode=cljs test^gclojure^step3 )
