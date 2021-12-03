import math

with open('day3/input.txt', 'r') as fp:
    lines = fp.readlines()

line_num = len(lines)
bits = [list(l.strip()) for l in lines]
# Transpose Matrix
bits_tr = list(map(list, zip(*bits)))


bit_counts = [sum(map(int,b)) for b in bits_tr]

gamma_rate = "".join([ "1" if x >= line_num/2 else "0" for x in bit_counts])
gamma_rate_int = int(gamma_rate,2)

epsilon_rate = "".join([ "0" if x >= line_num/2 else "1" for x in bit_counts])
epsilon_rate_int = int(epsilon_rate,2)

print(f"Gamma Rate: {gamma_rate_int}")
print(f"Epsilon Rate: {epsilon_rate_int}")
print(f"Result Part 1: {gamma_rate_int*epsilon_rate_int}")

## Part 2
oxy_bits = bits
oxy_bits_tr = bits_tr
i = 0
while len(oxy_bits) > 1:
    len(oxy_bits)
    vert_sum = sum(map(int,oxy_bits_tr[i]))
    if vert_sum*2 >= len(oxy_bits):
        oxy_bits = [line for line in oxy_bits if line[i] == "1"]
    else:
        oxy_bits = [line for line in oxy_bits if line[i] == "0"]
    oxy_bits_tr = list(map(list, zip(*oxy_bits)))
    i += 1
oxy_rating_bin = "".join(map(str,oxy_bits[0]))
oxy_rating = int(oxy_rating_bin, 2)

co2_bits = bits
co2_bits_tr = bits_tr
i = 0
while len(co2_bits) > 1:
    vert_sum = sum(map(int,co2_bits_tr[i]))
    if vert_sum*2 >= len(co2_bits): 
        co2_bits = [line for line in co2_bits if line[i] == "0"]
    else:
        co2_bits = [line for line in co2_bits if line[i] == "1"]
    co2_bits_tr = list(map(list, zip(*co2_bits)))
    i += 1
co2_rating_bin = "".join(map(str,co2_bits[0]))
co2_rating = int(co2_rating_bin, 2)

print(f"Oxygen generator rating: {oxy_rating}")
print(f"CO2 scrubber rating: {co2_rating}")
print(f"Life Support Rating: {oxy_rating*co2_rating}")

