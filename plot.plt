# vim: ft=gnuplot
set pm3d
set term pngcairo
set view map
set output "result.png"
splot "result.txt" with pm3d
set output "expect.png"
splot "expect.txt" with pm3d
