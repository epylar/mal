#!/bin/bash
set -e

( 
  cd .. 
  make test^grust^step0
  make REGRESS=1 OPTIONAL=0 test^grust^step1
  make REGRESS=1 OPTIONAL=0 test^grust^step2
  make REGRESS=1 OPTIONAL=0 test^grust^step3
  make REGRESS=1 OPTIONAL=0 test^grust^step4
  make REGRESS=1 OPTIONAL=0 test^grust^step5
  make REGRESS=1 OPTIONAL=0 test^grust^step6
  make REGRESS=1 OPTIONAL=0 test^grust^step7
)
