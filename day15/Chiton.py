#!/usr/bin/python3.9
import numpy as np

with open('day15/input.txt', 'r') as fp:
# with open('day15/testinput.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

heightMap = np.asarray([list(map(int,list(l))) for l in lines]).transpose()



def getNeighbors(pos, x_max, y_max):
    xi,yi = pos
    out = []
    if xi + 1 <= x_max: out.append((xi+1,yi  ))
    if xi - 1 >=     0: out.append((xi-1,yi  ))
    if yi + 1 <= y_max: out.append((xi  ,yi+1))
    if yi - 1 >=     0: out.append((xi  ,yi-1))
    return out

## hScore
def calcHeur(c_pos, g_pos):
    x,y = c_pos
    xg, yg = g_pos
    return ((xg - x)**2+(yg - y)**2)**0.5

def findPath(start, goal, localRisks):
    max_xi = localRisks.shape[0] - 1
    max_yi = localRisks.shape[1] - 1
    openSet = {start}

    ## gScore
    pathRisks = np.ones_like(localRisks)*999999
    pathRisks[start] = 0

    ## cameFrom
    parentArray = np.zeros_like(localRisks, dtype=object)

    ## fScore
    nodeRisks = np.zeros_like(localRisks,dtype=float)
    nodeRisks[start] = calcHeur(start, goal)
    while len(openSet) > 0:
        current = min(openSet,key=(lambda x: nodeRisks[x]))
        openSet.remove(current)
        if current == goal:
            return pathRisks[current], parentArray
        for nb in getNeighbors(current, max_xi, max_yi):
            pathRisk = pathRisks[current] + localRisks[nb]
            if pathRisk < pathRisks[nb]:
                parentArray[nb] = current
                pathRisks[nb] = pathRisk
                nodeRisks[nb] = pathRisk + calcHeur(nb,goal)
                openSet.add(nb)
    else:
        print("Search Failed")

max_xi = heightMap.shape[0] - 1
max_yi = heightMap.shape[1] - 1
start = (0,0)
firstGoal = (max_xi,max_yi)
goalCost, parents = findPath(start,firstGoal,heightMap)
# outMap = heightMap.copy()
# outMap[0,0] = 0
# tempPos = firstGoal
# while tempPos != start:
#     outMap[tempPos] = 0
#     tempPos = parents[tempPos]
# for line in outMap:
#     print("".join(map(str,line)))
print(f"Result Part 1: {goalCost}")

adjHeightMaps = []
for x in range(9):
    newHM = heightMap + x
    newHM = np.where(newHM > 9,newHM-9,newHM)
    adjHeightMaps.append(newHM)

lineHM = []
for y in range(5):
    lineHM.append(np.concatenate(adjHeightMaps[y:y+5],axis=0))
bigHeightMap = np.concatenate(lineHM, axis=1)

max_xi = bigHeightMap.shape[0] - 1
max_yi = bigHeightMap.shape[1] - 1
start = (0,0)
secondGoal = (max_xi,max_yi)
bigGoalCost, parents = findPath(start,secondGoal,bigHeightMap)
print(f"Result Part 2: {bigGoalCost}")
