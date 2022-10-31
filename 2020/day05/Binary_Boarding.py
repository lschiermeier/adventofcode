
with open('day05/input.txt', 'r') as fp:
# with open('day05/testinput.txt', 'r') as fp:
    lines = [l.strip() for l in fp.readlines()]

total_row_num = 128

maxID = 0
allRows = []
allCols = []
allIDs = []
for l in lines:
    row = int("".join(["0" if x == "F" else "1" for x in l[:7]]),2)
    col = int("".join(["0" if x == "L" else "1" for x in l[7:]]),2)
    id = row*8 + col
    maxID = maxID if maxID >= id else id
    allRows.append(row)
    allCols.append(col)
    allIDs.append(id)

print(f"Result Part 1: {maxID}")


allIDs.sort()

for idx, id in enumerate(allIDs):
    if idx + 1 == len(allIDs): break
    if id + 1 != allIDs[idx+1]:
        myID = id+1

print(f"Result Part 2: {myID}")

