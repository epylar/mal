#!/bin/bash

set -e

( 
  cd ..
  make RUST_BACKTRACE=1 OPTIONAL=0 test^grust^step9
)
