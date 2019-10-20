#!/bin/bash
set -e

( 
  cd .. 
  make test^grust^step0
  make OPTIONAL=0 test^grust^step1 
)
