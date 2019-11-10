#!/bin/bash

( cd .. &&
  make REGRESS=1 OPTIONAL=0 gclojure_MODE=clj test^gclojure^step4 )
