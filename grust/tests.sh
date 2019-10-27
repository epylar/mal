#!/bin/bash
set -e

( 
  cd .. 
  make test^grust^step0
  make OPTIONAL=0 test^grust^step1
  make OPTIONAL=0 test^grust^step2
  make OPTIONAL=0 test^grust^step3
  make OPTIONAL=0 test^grust^step4
)
