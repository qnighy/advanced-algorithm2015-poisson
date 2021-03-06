# vim: ft=gnuplot
input_file = sprintf("s%03d-result.txt", i)
output_file = sprintf("s%03d-result.png", i)
title_text = sprintf("Result of FDM (split = %d)", i)
set pm3d
set term pngcairo
set view map

set title title_text
set size square
set xlabel "x"
set ylabel "y"
set xrange [-0:1]
set yrange [-0:1]
set zrange [0:0.16]

set output output_file
splot input_file notitle with pm3d
