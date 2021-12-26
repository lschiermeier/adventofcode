import numpy as np

from numpy.lib.arraysetops import unique

with open('day24/input.txt', 'r') as fp:
# with open('day24/testinput.txt', 'r') as fp:
    lines = [x.strip().split() for x in fp.readlines()]

regs = "wxyz"


def runMONADp(inp, commands, state):
    for c in commands:
        reg = c[1]
        if c[0] == "inp": state[reg] = inp
        elif c[0] == "add": state[reg] += state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "mul": state[reg] *= state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "div": state[reg]  = state[reg] // (state[c[2]] if c[2] in regs else int(c[2]))
        elif c[0] == "mod": state[reg] %= state[c[2]] if c[2] in regs else int(c[2])
        elif c[0] == "eql": state[reg]  = state[reg] == (state[c[2]] if c[2] in regs else int(c[2]))
    return state


def findValid():
    inp_idxs = [idx for idx, l in enumerate(lines) if l[0] == "inp"] + [len(lines)]
    start_state = {reg: 0 for reg in regs}
    last_states = {"":start_state}
    for d in range(14):
        print(d,len(last_states))
        cmds = lines[inp_idxs[d]:inp_idxs[d+1]]
        new_states = {}
        for m in sorted(last_states.keys(),reverse=True):
            for x in range(9,0,-1):
                new_m = m + str(x)
                new_state = runMONADp(x,cmds,last_states[m].copy())
                if d == 13 and new_state["z"] == 0:
                    return new_m, new_state
                if not new_state in new_states.values():
                    new_states[new_m] = new_state
            pass
        last_states = new_states
    return "crashed", last_states

winning_monad, winning_state = findValid()

print(f"Result Part 1: {winning_monad}")
print(f"End State = {winning_state}")
pass