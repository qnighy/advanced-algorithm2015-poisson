#!/bin/sh
set -ue
convert -delay 10 -loop 0 *.png movie.gif
