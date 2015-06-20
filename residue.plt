# vim: ft=gnuplot
set term pngcairo

set title "FDM : stepsize -- residue-norm of analytic solution"
set size square
set xlabel "stepsize"
set ylabel "residue of analytic solution"
set xrange [0.001:100]
set yrange [1e-5:1]
set logscale x
set logscale y

set output "residue.png"
plot "overview.txt" using 2:5 notitle
