#!/bin/sh

for d in day[0-2][0-9]; do (echo ">>> $d"; cd $d/rs; cargo $*); done
