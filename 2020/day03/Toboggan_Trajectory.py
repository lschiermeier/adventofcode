from math import prod

def checkSlope(slope : tuple):
    steps_right, steps_down = slope
    trees_killed = 0
    hor_pos = 0
    for l in range(steps_down,len(lines),steps_down):
        hor_pos += steps_right
        if lines[l][hor_pos % len(lines[l])] == "#":
            trees_killed += 1
    return trees_killed

# with open('day03/testinput.txt', 'r') as fp:
with open('day03/input.txt', 'r') as fp:
    lines = [l.strip() for l in fp.readlines()]

slopes = [(a,1) for a in range(1,8,2)] + [(1,2)]
trees_killed = list(map(checkSlope,slopes))
print(slopes)

print(f"Right 1, down 1.: {trees_killed[0]}")
print(f"Right 3, down 1. (part 1): {trees_killed[1]}")
print(f"Right 5, down 1.: {trees_killed[2]}")
print(f"Right 7, down 1.: {trees_killed[3]}")
print(f"Right 1, down 2.: {trees_killed[4]}")

print(f"Result Part 2: {prod(trees_killed)}")
