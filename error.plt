# vim: ft=gnuplot
set term pngcairo

set title "FDM : stepsize -- error-norm of approximated solution"
set size square
set xlabel "stepsize"
set ylabel "residue of analytic solution"
set xrange [1e-4:10]
set yrange [1e-5:1]
set logscale x
set logscale y

set output "error.png"
plot "overview.txt" using 2:4 notitle
