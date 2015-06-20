#!/bin/bash
set -ue
er_options=()
diff_options=()
laplacian_options=()
for i in \
  `seq -f %03g 2 9` \
  `seq -f %03g 10 10 90` \
  `seq -f %03g 100 100 300`; do
  if [[ "$i" == 300 ]]; then
    j=100
  else
    j=0
  fi
  er_options+=(-delay 20 "s${i}-expect.png")
  er_options+=(-delay $((30 + $j)) "s${i}-result.png")
  diff_options+=(-delay $((50 + $j)) "diff-s${i}.png")
  laplacian_options+=(-delay $((50 + $j)) "laplacian-s${i}.png")
done
convert -loop 0 "${er_options[@]}" expect-and-result.gif
convert -loop 0 "${diff_options[@]}" diff.gif
convert -loop 0 "${laplacian_options[@]}" laplacian.gif
