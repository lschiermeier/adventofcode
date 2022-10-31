with open('day01/input.txt', 'r') as fp:
    entries = list(map(int,[l.strip() for l in fp.readlines()]))

for a in entries:
    for b in entries:
        if a + b == 2020:
            print(f"Result Part 1: {a*b}")
        for c in entries:
            if a + b + c == 2020:
                print(f"Result Part 2: {a*b*c}")