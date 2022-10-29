with open('day20/input.txt', 'r') as fp:
# with open('day20/testinput.txt', 'r') as fp:
    algorithm = [True if x == '#' else False for x in fp.readline().strip()]
    lines = [list(x.strip()) for x in fp.readlines()[1:]]

inputMap = dict()
for y, line in enumerate(lines):
    for x, pix in enumerate(line):
        inputMap[(x,y)] = True if pix == '#' else False
currentOutsideDefault = False

def get3x3(pos):
    xi,yi = pos
    for yd in range(-1,2,1):
        for xd in range(-1,2,1):
            yield (xi+xd, yi+yd)

def doEnhance(inMap):
    global currentOutsideDefault
    max_xi, max_yi = map(max,zip(*inMap.keys()))
    min_xi, min_yi = map(min,zip(*inMap.keys()))

    outMap = {}
    for y in range(min_yi-1,max_yi+2):
        for x in range(min_xi-1,max_xi+2):
            pos = (x,y)
            nbVal = [inMap.get(p, currentOutsideDefault) for p in get3x3(pos) ]
            algIdx = int("".join(['1' if x else '0' for x in nbVal]),2)
            outMap[pos] = algorithm[algIdx]
    currentOutsideDefault = not currentOutsideDefault
    return outMap

def printMap(inMap, fp):
    max_xi, max_yi = map(max,zip(*inMap.keys()))
    min_xi, min_yi = map(min,zip(*inMap.keys()))
    for y in range(min_yi,max_yi+1):
        for x in range(min_xi,max_xi+1):
            print("#" if inMap[(x,y)] else ".",end='', file=fp)
        print("", file=fp)

workMap = inputMap.copy()
workMap = doEnhance(workMap)
workMap = doEnhance(workMap)



for i in range(50):
    if i % 5 == 0:
        print("enhance step:", int(i+1))
    workMap = doEnhance(workMap)
    if i == 1:
        alightPixels = sum(workMap.values())
        print(f"Result Part 1: {alightPixels}")

alightPixels = sum(workMap.values())

print(f"Result Part 2: {alightPixels}")
