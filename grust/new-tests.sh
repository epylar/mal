#!/bin/bash

set -e

( 
  cd ..
  make OPTIONAL=0 test^grust^step5
)
