import re

with open('day07/input.txt', 'r') as fp:
# with open('day07/testinput.txt', 'r') as fp:
    lines = [l.strip() for l in fp.readlines()]

rule_list = [re.split(" bag[s]?[\.|,]? ?(?:contain )?", l)[:-1] for l in lines]
can_contain = {x[0]:[(int(bl[0]),bl[1]) for b in x[1:] if not (bl :=b.split(maxsplit=1))[0] == "no" ] for x in rule_list}

can_be_in = {}
for x in rule_list:
    if x[1] == "no other":
        continue
    for b in x[1:]:
        times, bag = b.split(maxsplit=1)
        # can_be_in.get(bag,list()).append((x[0],int(times)))
        can_be_in[bag] = can_be_in.get(bag,list()) + [(x[0],int(times))]

inner_most = "shiny gold"
old_set = set()
new_set = {inner_most}

while len(new_set) > len(old_set):
    temp_set = set()
    for bag in new_set - old_set:
        if bag in can_be_in.keys():
            temp_set = temp_set | { b for b,_ in can_be_in[bag]}
    old_set = new_set
    new_set = old_set | temp_set
new_set = new_set - {inner_most}

print(f"Result Part 1: {len(new_set)}")



pass