with open('day06/input.txt', 'r') as fp:
# with open('day06/testinput.txt', 'r') as fp:
    lines = [l.strip() for l in fp.readlines()]
people = [[int(x,36) - 10 for x in l] for l in lines]


groups = []
new_group = [False]*26
for person in people:
    if person == []:
        groups.append(new_group)
        new_group = [False]*26
    for i in person:
        new_group[i] = True
else:
    groups.append(new_group)

print(f"Result Part 1: {sum(map(sum,groups))}")

groups = []
new_group = people[0]
for idx, person in enumerate(people):
    if person == []:
        groups.append(len(new_group))
        new_group = people[idx+1]
    else: 
        temp_new_group = new_group.copy()
        for a in new_group:
            if not a in person:
                temp_new_group.remove(a)
        new_group = temp_new_group
else:
    groups.append(len(new_group))

print(f"Result Part 2: {sum(groups)}")
