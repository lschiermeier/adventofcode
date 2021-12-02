aim = 0
hor_pos = 0
depth = 0
lines = None
with open('day2/input.txt', 'r') as fp:
    lines = fp.readlines()
commands = [x.strip().split(" ") for x in lines]

for c in commands:
    if c[0] == "forward":
        hor_pos += int(c[1])
        depth += int(c[1]) * aim
    elif c[0] == "down":
        aim += int(c[1])
    else:
        aim -= int(c[1])




print(f"Current Aim: {aim}")
print(f"Depth Pos: {depth}")
print(f"Horizontal Pos: {hor_pos}")
print(f"Result: {depth*hor_pos}")

