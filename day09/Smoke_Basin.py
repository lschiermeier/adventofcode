import numpy as np
from math import prod

with open('day09/input.txt', 'r') as fp:
# with open('day09/testinput.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

height_map = np.asarray([list(map(int,list(l))) for l in lines])

yis = list(range(height_map.shape[0]))
xis = list(range(height_map.shape[1]))
max_yi = max(yis)
max_xi = max(xis)

def check_local_minimum(map,xi,yi):
    state = True
    if xi < max_xi:
        state = state and map[yi,xi] < map[yi,xi+1]
    if xi > 0:
        state = state and map[yi,xi] < map[yi,xi-1]
    if yi < max_yi:
        state = state and map[yi,xi] < map[yi+1,xi]
    if yi > 0:
        state = state and map[yi,xi] < map[yi-1,xi]
    return state

def find_all_minima(map):
    minima = np.zeros_like(map)
    for xi in xis:
        for yi in yis:
            minima[yi,xi] = check_local_minimum(height_map,xi,yi)
    return minima

all_minima = find_all_minima(height_map)

min_vals = np.where(all_minima,height_map,-1)
risk_grid = min_vals + 1
risk_sum = np.sum(risk_grid)

print(f"Result Part 1: {risk_sum}")

def find_basin_size(map,idx):
    to_be_expanded = {idx}
    part_of_basin = {idx}

    while len(to_be_expanded) > 0:
        next_expansion = set()
        for yi, xi in to_be_expanded:
            if xi < max_xi and not (yi,xi+1) in part_of_basin and map[yi,xi+1] < 9:
                next_expansion.add((yi,xi+1))
            if xi > 0      and not (yi,xi-1) in part_of_basin and map[yi,xi-1] < 9:
                next_expansion.add((yi,xi-1))
            if yi < max_yi and not (yi+1,xi) in part_of_basin and map[yi+1,xi] < 9:
                next_expansion.add((yi+1,xi))
            if yi > 0      and not (yi-1,xi) in part_of_basin and map[yi-1,xi] < 9:
                next_expansion.add((yi-1,xi))
        part_of_basin = part_of_basin | next_expansion
        to_be_expanded = next_expansion
    return len(part_of_basin)


min_idxs = np.where(all_minima)

basin_sizes = [find_basin_size(height_map,i) for i in zip(*min_idxs)]

dangerzones = prod(sorted(basin_sizes)[-3:])
print(f"Result Part 2: {dangerzones}")

