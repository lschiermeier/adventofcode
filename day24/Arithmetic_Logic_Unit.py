from os import X_OK
import numpy as np
from bisect import insort

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

inp_idxs = [idx for idx, l in enumerate(lines) if l[0] == "inp"] + [len(lines)]

start_state = {reg: 0 for reg in regs}
monads = [""]
possible_states = [{} for _ in range(15)]
possible_states[0][""] = start_state
found_best = False
counter = 0
while not found_best:
    counter += 1
    if not counter % 1000:
        print(counter, len(monads),monads[-1])
    monad = monads.pop()
    d = len(monad)
    state = possible_states[d][monad]
    new_monads = []
    cmds = lines[inp_idxs[d]:inp_idxs[d+1]]
    for x in range(9,0,-1):
        new_m = monad + str(x)
        new_state = runMONADp(x, cmds, state.copy())
        if  new_state["z"] < 1000000:
            if not new_state in possible_states[d+1].values() and len(new_m) < 14:
                possible_states[d+1][new_m] = new_state
                new_monads.insert(0,new_m)
            elif len(new_m) == 14 and new_state["z"] == 0:
                found_best = True
                winning_monad = new_m
                winning_state = new_state
                break
    monads = monads + new_monads

print(f"Result Part 1: {winning_monad}")
print(f"End State = {winning_state}")
pass