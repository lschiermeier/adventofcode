# with open('day14/testinput.txt', 'r') as fp:
with open('day14/input.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

template = lines[0]
rules = {x.split(" -> ")[0]:x.split(" -> ")[1] for x in lines[2:]}

# naive
def doStep(polymer):
    newPolymer = polymer[0]
    for i in range(len(polymer)-1):
        newPolymer = newPolymer + rules[polymer[i:i+2]] + polymer[i+1]
    return newPolymer

part1_polymer = template
for i in range(10):
    part1_polymer = doStep(part1_polymer)

elements = list(set(rules.values()))
occurrences = [part1_polymer.count(e) for e in elements]
mc_num = max(occurrences)
lc_num = min(occurrences)
mc_element = elements[occurrences.index(mc_num)]
lc_element = elements[occurrences.index(lc_num)]

print(f"Result Part 1: {mc_num-lc_num}")
print(f"{mc_element} occurred: {mc_num} times")
print(f"{lc_element} occurred: {lc_num} times")

