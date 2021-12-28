def create_stacks(lines):
    """creates stacks as dict of cols highest AP last"""
    row_idxs = list(range(2,4)) if len(lines) < 6 else list(range(2,6))
    row_num = len(row_idxs)

    stacks = {}
    for x in col_idxs:
        stacks[x] =[lines[y][x] for y in reversed(row_idxs)]
    return stacks, row_num

def find_valid_moves(stacks, toprow):
    pass

def judge_state(stacks, toprow):
    for i, s in stacks.items():

        pass
    pass

def find_best_moves(lines):
    stacks, row_num = create_stacks()
    toprow = {}
    pass

def part1():
    with open('day23/input.txt', 'r') as fp:
        lines = [list(x) for x in fp.readlines()]
    print(create_stacks(lines))
    # costs =  find_best_moves(lines)


def part2():
    with open('day23/input2.txt', 'r') as fp:
        lines = [list(x) for x in fp.readlines()]
    print(create_stacks(lines))


col_idxs = [3,5,7,9]
col_targets = list("ABCD")
toprow_i = 1
topcol_i = [x for x in range(1,13) if not x in col_idxs]


print(f"Result Part 1: {part1()}")
print(f"Result Part 2: {part2()}")