#!/bin/bash

( cd .. &&
  make OPTIONAL=0 GCLOJURE_mode=clj test^gclojure^step0 )
