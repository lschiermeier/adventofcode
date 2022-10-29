with open('day08/input.txt', 'r') as fp:
    lines = [x.strip().split(" | ") for x in fp.readlines()]

#               0, 1, 2, 3, 4, 5, 6, 7, 8, 9
pattern_lens = (6, 2, 5, 5, 4, 5, 6, 3, 7, 6)
segments = set("abcdefg")
simple_len2num = {2:1, 4:4, 3:7, 7:8}
normal_patterns = ("abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg","abcdfg")

normal_patterns_five = {p for p in normal_patterns if len(p) == 5}
options_five = [2,3,5]
# ['acdeg', 'acdfg', 'abdfg']
# adg, top, mid, bot
# ce, cf, bf: 5
# e unique: bot
normal_patterns_six = {p for p in normal_patterns if len(p) == 6}
options_six = [0,6,9]
# ['abcefg', 'abdefg', 'abcdfg']
# abfg common, top, top left, bot, bot right
# ce: 0, de, cd
# d: the one missing one of the three common in five is 0!
# e: top right
# 

displays = []

simple_outs = 0


for digits, value in lines:
    unclear_patterns = list(map(set,digits.split()))
    value_patterns = list(map(lambda x: "".join(sorted(x)),value.split()))

    patterns_five = [p for p in unclear_patterns if len(p) == 5]
    patterns_six = [p for p in unclear_patterns if len(p) == 6]
    pattern_to_num = {"".join(sorted(p)): simple_len2num[len(p)] for p in unclear_patterns if len(p) in simple_len2num.keys()}

    simple_outs += sum([1 for x in value_patterns if x in pattern_to_num.keys()])
    display = (patterns_five, patterns_six, pattern_to_num, value_patterns)
    displays.append(display)

print(f"Result Part 1: {simple_outs}")

all_values = []
for p5, p6, p2n, values in displays:
    n2p = {v:set(k) for k,v in p2n.items()}
    # 2, 3, 5
    # adg, top, mid, bot
    common_of_five = set(p5[0]) & set(p5[1]) & set(p5[2])
    # 0, 6, 9
    # abfg, top, top left, bot, bot right
    common_of_six = set(p6[0]) & set(p6[1]) & set(p6[2])
    

    # known 1, 4, 7, 8
    bot_top = common_of_five & common_of_six
    mid = common_of_five - common_of_six
    # bot right, top left
    bf = common_of_six - bot_top

    eight = n2p[8]
    one = n2p[1]

    zero  = eight - mid
    three = one | bot_top | mid
    five  = mid | bot_top | bf
    two   = eight - bf
    six   = eight - (two & one)
    nine  = n2p[4] | bot_top
    # known 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    p2n["".join(sorted(zero)) ] = 0
    p2n["".join(sorted(three))] = 3
    p2n["".join(sorted(five)) ] = 5
    p2n["".join(sorted(two))  ] = 2
    p2n["".join(sorted(six))  ] = 6
    p2n["".join(sorted(nine)) ] = 9
    
    num = int("".join([str(p2n[x]) for x in values]))
    all_values.append(num)

sum_all_v = sum(all_values)
print(f"Result Part 2: {sum_all_v}")
