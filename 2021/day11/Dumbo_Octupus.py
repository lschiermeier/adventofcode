#!/usr/bin/python3.9
import numpy as np
from numpy.lib.twodim_base import tri

with open('day11/input.txt', 'r') as fp:
# with open('day11/testinput.txt', 'r') as fp:
    lines = [list(x.strip()) for x in fp.readlines()]

sourceMap = np.asarray(lines, dtype=np.int64)

def getEightNeighbors(pos,map):
    y_bound,x_bound = map.shape
    yi,xi = pos
    nbs = []
    for xd in range(-1,2,1):
        for yd in range(-1,2,1):
            if xi + xd >= 0 and xi + xd < x_bound:
                if yi + yd >= 0 and yi + yd < y_bound:
                    nbs.append((yi+yd,xi+xd))
    nbs.remove(pos)
    return nbs

def doStep(octos):
    octos += 1
    triggered_octos = []
    while len(triggers := [x for x in zip(*np.nonzero(octos > 9)) if not x in triggered_octos]) > 0:
        for t in triggers:
            blubb = tuple(zip(*getEightNeighbors(t,octos)))
            octos[blubb] += 1
        triggered_octos += triggers
    octos = np.where(octos > 9, 0, octos)
    # for line in octos:
        # print("".join(map(lambda x: " {:1d}".format(x),line)))
    # pass
    return octos, triggered_octos

octoMap = sourceMap.copy()
num_triggerings = 0
for i in range(100):
    octoMap, triggered_octos = doStep(octoMap)
    num_triggerings += len(triggered_octos)
print(f"Result Part 1: {num_triggerings}")

found = False
step = 0
octoMap = sourceMap.copy()
num_triggerings = 0
while not found:
    step += 1
    octoMap, triggered_octos = doStep(octoMap)
    found = octoMap.size == len(triggered_octos)
    num_triggerings += len(triggered_octos)
print(f"Result Part 2: {step}, {num_triggerings}")
