#!/usr/bin/sage
from sage.all_cmdline import *

if len(sys.argv) < 2:
    print("Group: ", end="")
    grp_name = input().strip()
else:
    grp_name = sys.argv[1].strip()

G = eval(grp_name)
tom = matrix(ZZ, gap(G).TableOfMarks().MatTom())
N = tom.nrows()

# tom^i_j = |Hom(G/H_j, G/H_i)| = X(G/H_i)(H_j)
# H_i is subconjugate to H_j <==> |Hom(G/H_i,G/H_j)| > 0 <==> tom^i_j > 0 ==> i <= j.
# tom_ij > 0 ==> i >= j

# X(\sum_i n_i G/H_i)(H_j) = \sum_i n_i X(G/H_i)(H_j) = \sum_i n_i t^i_j = (n * tom)_j

ds = [[ tom.row(i).pairwise_product(tom.row(j)) / tom for j in range(0,N) ] for i in range(0,N)]
ds = list(map(lambda r: list(map(lambda v: [k for k in range(0,N) if v[k] != 0], r)), ds))

f = open(f"data/{grp_name}.txt", "w")
f.write(f"{N}\n")
f.write(str(ds))
f.close()

print(f'Produced data/{grp_name}.txt')

