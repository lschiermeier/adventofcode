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

# good
expansions = {k: k[0]+v+k[1] for k,v in rules.items()}
additions = {k:[v[0:2],v[1:3]] for k,v in expansions.items()}
occurrencesTemplate = [template.count(e) for e in elements]

ruleCountsTemplate = {}
for i in range(len(template)-1):
    k = template[i:i+2]
    ruleCountsTemplate[k] = ruleCountsTemplate.get(k,0) + 1

def doBetterStep(ruleCounts):
    newRuleCounts = {}
    for k, v in ruleCounts.items():
        a, b = additions[k]
        newRuleCounts[a] = newRuleCounts.get(a,0) + v
        newRuleCounts[b] = newRuleCounts.get(b,0) + v
    return newRuleCounts

def convertRuleCounts(ruleCounts):
    elementCounts = {e:0 for e in elements}
    for k,v in ruleCounts.items():
        elementCounts[k[0]] += v
    elementCounts[template[-1]] +=1
    return elementCounts

ruleCountsPart2 = ruleCountsTemplate.copy()
for i in range(40):
    ruleCountsPart2 = doBetterStep(ruleCountsPart2)

eCounts = convertRuleCounts(ruleCountsPart2)

c2e = {v:k for k,v in eCounts.items()}
mc_num = max(c2e.keys())
lc_num = min(c2e.keys())
mc_element = c2e[mc_num]
lc_element = c2e[lc_num]

print(f"Result Part 2: {mc_num-lc_num}")
print(f"{mc_element} occurred: {mc_num} times")
print(f"{lc_element} occurred: {lc_num} times")

