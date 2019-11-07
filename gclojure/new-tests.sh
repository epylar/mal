#!/bin/bash

( cd .. &&
  make OPTIONAL=0 gclojure_MODE=cljs test^gclojure^step1 )
