#!python3.9
import itertools as it
import numpy as np
from numpy.lib.function_base import flip

# with open('day13/testinput.txt', 'r') as fp:
with open('day13/input.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

split_index = lines.index("")
dots = [(int(x),int(y)) for x,y in [l.split(",") for l in lines[:split_index]]]
folds = [(ax, int(num)) for ax,num in [l[11:].split("=") for l in lines[split_index+1:]]]
max_x = max([x for ax, x in folds if ax == "x"])*2+1
max_y = max([y for ax, y in folds if ax == "y"])*2+1

dot_map = np.zeros((max_x,max_y), dtype=bool)
dot_map[tuple(zip(*dots))] = True

def doFold(fold, map):
    ax, xy = fold
    if ax == "y":
        flipped = np.flip(map[:,xy+1:],1)
        map = map[:,:xy] | flipped
    else:
        flipped = np.flip(map[xy+1:,:],0)
        map = map[:xy,:] | flipped
    return map


dot_map = doFold(folds[0],dot_map)
dotsum = np.sum(dot_map)
print(f"Result Part 1: {dotsum}")

for f in folds[1:]:
    dot_map = doFold(f,dot_map)

dotsum = np.sum(dot_map)
print(f"Result Part 2: {dotsum}")

for line in dot_map.transpose():
    print("".join(["#" if x else " " for x in line]))
