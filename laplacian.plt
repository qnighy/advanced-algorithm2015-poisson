# vim: ft=gnuplot
input_file = sprintf("equation-s%03d.txt", i)
output_file = sprintf("laplacian-s%03d.png", i)
title_text = sprintf("Given laplacian (split = %d)", i)
set pm3d
set term pngcairo
set view map

set title title_text
set size square
set xlabel "x"
set ylabel "y"
set xrange [-0:1]
set yrange [-0:1]
set cbrange [0.0:0.7]

set output output_file
splot input_file using 1:2:3 notitle with pm3d
