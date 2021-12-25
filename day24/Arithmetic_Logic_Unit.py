import numpy as np

with open('day24/input.txt', 'r') as fp:
# with open('day24/testinput.txt', 'r') as fp:
    lines = [x.strip().split() for x in fp.readlines()]

regs = "wxyz"


def runMONADp(inps, commands):
    state = {reg: 0 for reg in regs}
    inps = iter(inps)
    for c in commands:
        reg = c[1]
        if c[0] == "inp": state[reg] = next(inps)
        elif c[0] == "add": state[reg] += state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "mul": state[reg] *= state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "div": state[reg]  = state[reg] // state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "mod": state[reg] %= state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "eql": state[reg]  = state[reg] == (state[c[2]] if c[2] in regs else int(c[2]))
    return state["z"]

grid = np.zeros((14,9), dtype=np.int64)
inp_idxs = [idx for idx, l in enumerate(lines) if l[0] == "inp"] + [len(lines)]

for x in range(0,14):
    for y in range(0,9):
        grid[x,y] = runMONADp([y+1],lines[inp_idxs[x]:inp_idxs[x+1]])

print(grid)

grid2 = [runMONADp([x]*14, lines) for x in range(1,10)]

print(grid2)

pass
# print(f"Result Part 1: {m[idx]}")