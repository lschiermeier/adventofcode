with open('day04/input.txt', 'r') as fp:
# with open('day04/testinput.txt', 'r') as fp:
    lines = [l.strip() for l in fp.readlines()]

passports = []
possible_fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid","cid"]
needed_fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid"]
valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
new_passport = {}
for l in lines:
    if l == '':
        passports.append(new_passport)
        new_passport = {}
        continue
    for field in l.split():
        key, val = field.split(":")
        new_passport[key] = val
passports.append(new_passport)

def checkValid_part1(passport:dict):
    for f in needed_fields:
        if not f in passport.keys():
            return False
    return True

def checkValid_part2(pp):
    for f in needed_fields:
        if not f in pp.keys(): return False
    if len(pp["iyr"]) != 4 or (int(pp["byr"]) < 1920 or int(pp["byr"]) > 2002): return False
    if len(pp["iyr"]) != 4 or (int(pp["iyr"]) < 2010 or int(pp["iyr"]) > 2020): return False
    if len(pp["eyr"]) != 4 or (int(pp["eyr"]) < 2020 or int(pp["eyr"]) > 2030): return False
    hgt = pp["hgt"]
    if hgt[-2:] == "cm" and (int(hgt[:-2]) < 150 or int(hgt[:-2]) > 193): return False
    if hgt[-2:] == "in" and (int(hgt[:-2]) < 59 or int(hgt[:-2]) > 76): return False
    if hgt[-2:] != "cm" and hgt[-2:] != "in": return False
    hcl = pp["hcl"]
    if hcl[0] == "#" and len(hcl) == 7:
        try: int(hcl[1:], base=16)
        except ValueError: return False
    else: return False
    if not pp["ecl"] in valid_ecl: return False
    if len(pp["pid"]) != 9: return False
    return True

valid_counter = sum(list(map(checkValid_part1,passports)))
print(f"Result Part 1: {valid_counter}")
valid_counter = sum(list(map(checkValid_part2,passports)))
good_pp = list(filter(checkValid_part2, passports))
bad_pp = list(filter(lambda x: not checkValid_part2(x) and len(x) >= 7, passports))
print(f"Result Part 2: {valid_counter}")

