with open('day08/input.txt', 'r') as fp:
    lines = [x.strip().split(" | ") for x in fp.readlines()]

#               0, 1, 2, 3, 4, 5, 6, 7, 8, 9
pattern_lens = (6, 2, 5, 5, 4, 5, 6, 3, 7, 6)
simple_len2num = {2:1, 4:4, 3:7, 7:8}

displays = []

simple_outs = 0

for digits, value in lines:
    unclear_patterns = list(map(lambda x: "".join(sorted(x)),digits.split()))
    value_patterns = list(map(lambda x: "".join(sorted(x)),value.split()))

    patterns_five = [p for p in unclear_patterns if len(p) == 5]
    patterns_six = [p for p in unclear_patterns if len(p) == 6]
    simple_patterns = {p: simple_len2num[len(p)] for p in unclear_patterns if len(p) in simple_len2num.keys()}
    
    simple_outs += sum([1 for x in value_patterns if x in simple_patterns.keys()])



print(f"Result Part 1: {simple_outs}")