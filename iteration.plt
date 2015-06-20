# vim: ft=gnuplot
set term pngcairo

set title "FDM : stepsize -- number of iteration"
set size square
set xlabel "stepsize"
set ylabel "number of iteration"
#set xrange [-0:1]
#set yrange [-0:1]
set xrange [1e-3:1]
set yrange [1:1000]
set logscale x
set logscale y

set output "iteration.png"
plot "overview.txt" using 2:3 notitle
