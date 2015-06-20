#!/bin/bash
set -ue
# cargo run --release
for i in `seq 2 9` `seq 10 10 90` `seq 100 100 300`; do
  gnuplot -e "i=${i}" result.plt
  gnuplot -e "i=${i}" expect.plt
  gnuplot -e "i=${i}" diff.plt
  gnuplot -e "i=${i}" laplacian.plt
  gnuplot -e "i=${i}" grad_x.plt
  gnuplot -e "i=${i}" grad_y.plt
done
gnuplot error.plt
gnuplot residue.plt
gnuplot iteration.plt
