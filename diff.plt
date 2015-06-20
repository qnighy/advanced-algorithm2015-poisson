# vim: ft=gnuplot
input_file = sprintf("diff-s%03d.txt", i)
output_file = sprintf("diff-s%03d.png", i)
title_text = sprintf("Error of FDM (split = %d)", i)
set pm3d
set term pngcairo
set view map

set title title_text
set size square
set xlabel "x"
set ylabel "y"
set xrange [-0:1]
set yrange [-0:1]
set cbrange [1e-10:0.1]
set logscale zcb

set output output_file
splot input_file notitle with pm3d
