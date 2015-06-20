#!/bin/sh
set -ue
files_sorted=$(ls *-result.png *-expect.png | sort)
convert -delay 10 -loop 0 ${files_sorted} movie.gif
