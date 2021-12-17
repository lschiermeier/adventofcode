#!python3.10

from types import LambdaType
from statistics import median

with open('day10/input.txt', 'r') as fp:
# with open('day10/testinput.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

char_pairs = {"(":")","[":"]","{":"}","<":">"}
openers = list(char_pairs.keys())
closers = list(char_pairs.values())

corruptor_score = {")":3, "]":57, "}":1197, ">": 25137}



def test_corruption(line) -> tuple[bool, object]:
    cs = list(line)
    stack = []
    for c in cs:
        if c in openers:
            stack.append(c)
        else:
            if len(stack) == 0:
                return True, c
            opener = stack.pop()
            if c != char_pairs[opener]:
                return True, c
    return False, stack




incomplete = []
stacks = []
corruptors = []
for l in lines:
    corrupted, obj = test_corruption(l)
    if not corrupted:
        incomplete.append(l)
        stacks.append(obj)
    else:
        corruptors.append(obj)

error_score = sum(map(lambda x: corruptor_score[x],corruptors))

print(f"Result Part 1: {error_score}")

completer_score = {")":1, "]":2, "}":3, ">":4}

scores = []
for l, s in zip(incomplete, stacks):
    scores.append(sum([completer_score[char_pairs[c]]*5**i for i,c in enumerate(s)]))
completion_score = median(scores)

print(f"Result Part 2: {completion_score}")


pass