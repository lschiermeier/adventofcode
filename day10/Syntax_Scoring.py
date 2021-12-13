# with open('day10/input.txt', 'r') as fp:
from os import confstr_names


with open('day10/testinput.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

char_pairs = {"(":")","[":"]","{":"}","<":">"}

illegal_points = {")":3, "]":57, "}":1197, ">": 25137}


def corruption_rec(c, cs):
    match cs:
        case [closing, next, *cs_cont] if closing == char_pairs[c]:
            print(c,closing, next, cs_cont)
            return corruption_rec(next, cs_cont)
        case [closing, *cs_cont] if closing in char_pairs.values():
            return False, closing
        case [opening ,*cs_cont] if opening in char_pairs.keys():
            functional, cs_rest = corruption_rec(opening, cs_cont)
        case _:
            pass



def is_corrupted(line):
    c, cs = line[0], line[1:]
    while len(cs) > 0 and functional:
        functional, cs = corruption_rec(c,cs)
    return functional, cs

corruption_rec("(",list(")[]()()"))