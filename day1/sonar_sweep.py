#!python3

inc_counter = 0
part_two_counter = 0
with open('day1/input.txt', 'r') as fp:
    prevprevprevline = fp.readline()
    prevprevline = fp.readline()
    if(int(prevprevprevline) < int(prevprevline)):
        inc_counter += 1
    prevline = fp.readline()
    if(int(prevprevline) < int(prevline)):
        inc_counter += 1
    # print(prevline)
    line = fp.readline()
    while line:
        # print(line)
        if(int(prevline) < int(line)):
            inc_counter += 1
        sumA = int(prevprevprevline) + int(prevprevline) + int(prevline)
        sumB = int(prevprevline) + int(prevline) + int(line)
        if(sumB > sumA):
            part_two_counter += 1
        prevprevprevline = prevprevline
        prevprevline = prevline
        prevline = line
        line = fp.readline()
        

print("Problem 1 output: {}".format(inc_counter))
print("Problem 2 output: {}".format(part_two_counter))

