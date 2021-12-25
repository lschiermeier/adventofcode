import numpy as np

with open('day25/input.txt', 'r') as fp:
# with open('day25/testinput.txt', 'r') as fp:
    lines = [list(x.strip()) for x in fp.readlines()]

source = np.asarray(lines)
grid = np.where(source==">", 1, 0)
grid = np.where(source=="v", 2, grid)

def getGoals(grid, poss, dir):
    y_len, x_len = grid.shape
    ys, xs = poss
    if dir == 1:
        xs = np.fromiter((x+1 if x+1 < x_len else 0 for x in xs), np.int64)
    else:
        ys = np.fromiter((y+1 if y+1 < y_len else 0 for y in ys), np.int64)
    return ys, xs

def doMove(grid, herd):
    other_herd = 1 if herd == 2 else 2
    new_grid = np.where(grid==other_herd, other_herd, 0)
    members = np.where(grid==herd)
    goals = getGoals(grid, members, herd)
    selector = grid[goals] == 0
    new_locs = np.where(selector, goals, members)
    new_grid[new_locs[0],new_locs[1]] = herd
    return new_grid, sum(selector)

step = 0
has_moved = 1
while has_moved:
    step += 1
    grid, east_moves = doMove(grid,1)
    grid, south_moves = doMove(grid,2)
    # print(f"Step {step}:",east_moves,south_moves)
    has_moved = east_moves + south_moves

print(f"Result Part 1: {step}")
print(grid)

pass