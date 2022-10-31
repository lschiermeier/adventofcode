with open('day02/input.txt', 'r') as fp:
    lines = [l.strip().split(" ") for l in fp.readlines()]

valid_counter = 0
for range, letter, pw in lines:
    left, right  = list(map(int,range.split("-")))
    letter_count = pw.count(letter[0])
    if letter_count >= left and letter_count <= right:
        valid_counter += 1

print(f"Result part 1: {valid_counter}")

valid_counter = 0
for range, letter, pw in lines:
    left, right = list(map(int,range.split("-")))
    if (pw[left-1] == letter[0]) ^ (pw[right-1] == letter[0]):
        valid_counter += 1

print(f"Result part 2: {valid_counter}")
