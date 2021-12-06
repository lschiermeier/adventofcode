depth = 0
hor_pos = 0
lines = None
with open('day02/input.txt', 'r') as fp:
    lines = fp.readlines()
commands = [x.strip().split(" ") for x in lines]

for c in commands:
    if c[0] == "forward":
        hor_pos += int(c[1])
    elif c[0] == "down":
        depth += int(c[1])
    else:
        depth -= int(c[1])




print(f"Depth Pos: {depth}")
print(f"Horizontal Pos: {hor_pos}")
print(f"Result: {depth*hor_pos}")

