# class Fish:
#     timer = -1

#     def __init__(self, t: int):
#         self.timer = t


import numpy as np

# with open('day06/testinput.txt', 'r') as fp:
with open('day06/input.txt', 'r') as fp:
    starting_fishes = list(map(int,fp.readline().strip().split(",")))

print(f"We begin with {len(starting_fishes)} Fishes")

# naive
fishes = starting_fishes
for d in range(0,80):
    babies = [8 for f in fishes if f == 0]
    fishes = [f-1 for f in fishes if f > 0] + [6 for f in fishes if f == 0]
    fishes += babies

    if d == 79 or d == 255:
        print(f"In Generation {d+1}: {len(babies)} Babies were born for a total of {len(fishes)}")

# part 2
fish_counters = [starting_fishes.count(i) for i in range(0,9)]
for d in range(0,256):
    new_fishcounters = [0]*9
    for i, f in enumerate(fish_counters):
        if i == 0:
            new_fishcounters[8] = f
            new_fishcounters[6] = f
        elif i == 7:
            new_fishcounters[6] += f
        else:
            new_fishcounters[i-1] = f
    if (d+1 % 10) == 0 or d == 255:
        print(f"In Generation {d+1}: {sum(new_fishcounters) - sum(fish_counters)} Babies were born for a total of {sum(new_fishcounters)}")
    fish_counters = new_fishcounters
