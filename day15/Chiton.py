#!/usr/bin/python3.9
import numpy as np
from math import prod

with open('day15/input.txt', 'r') as fp:
# with open('day15/testinput.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

height_map = np.asarray([list(map(int,list(l))) for l in lines]).transpose()

xis = list(range(height_map.shape[0]))
yis = list(range(height_map.shape[1]))
max_xi = max(xis)
max_yi = max(yis)

class Node():
    def __init__(self,pos,*args):
        self.pos = pos
        self.localRisk = height_map[pos]
        if isinstance(args[0],Node):
            self.parent = args[0]
            self.pathRisk = self.parent.pathRisk + self.localRisk
            self.goal = self.parent.goal
        else:
            self.parent = None
            self.pathRisk = 0
            self.goal = args[0]
        self.heurRisk = ((self.goal[0]-pos[0])**2+(self.goal[1]-pos[1])**2)**0.5
        self.nodeRisk = float(self.pathRisk) + self.heurRisk

    def getNodeRisk(some):
        return some.nodeRisk
    def getNodeRisk(self):
        return self.nodeRisk
    def __eq__(self, __o: object) -> bool:
        if isinstance(__o, Node):
            return self.pos == __o.pos
        return False


def findPath(startPos, goalPos):
    start = [Node(startPos,goalPos)]
    openList = sorted(start, key=Node.getNodeRisk)
    closedList = sorted([],key=Node.getNodeRisk)
    while len(openList) > 0:
        toExpand = openList.pop(0)
        xi,yi = toExpand.pos
        newNodes = []
        if xi < max_xi:
            newNodes.append(Node((xi+1,yi),toExpand))
        if xi > 0:
            newNodes.append(Node((xi-1,yi),toExpand))
        if yi < max_yi:
            newNodes.append(Node((xi,yi+1),toExpand))
        if yi > 0:
            newNodes.append(Node((xi,yi-1),toExpand))
        for n in newNodes:
            if n.pos == goalPos:
                return n
            other = sorted([x for x in closedList + openList if x == n],key=Node.getNodeRisk)
            if not other == []:
                if other[0].getNodeRisk() <= n.getNodeRisk():
                    # print(other[0].getNodeRisk(),n.getNodeRisk())
                    continue
            openList.append(n)
            openList.sort(key=Node.getNodeRisk)
        closedList.append(toExpand)
        closedList.sort(key=Node.getNodeRisk)
    print("Search Failed")
    return -1


goalNode = findPath((0,0),(max_xi,max_yi))
print(f"Result Part 1: {goalNode.pathRisk}")
tempNode = goalNode
outMap = height_map.copy()
outMap[goalNode.pos] = 0
outMap[0,0] = 0
while tempNode.parent != None:
    tempNode = tempNode.parent
    outMap[tempNode.pos] = 0


for line in outMap:
    print("".join(map(str,line)))
