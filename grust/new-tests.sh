#!/bin/bash

set -e

( 
  cd ..
  make REGRESS=1 RUST_BACKTRACE=1 OPTIONAL=0 test^grust^step9
)
