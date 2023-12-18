import sys
import re

lines = [line.rstrip() for line in sys.stdin]

ports = []
index = 0
for line in lines:
    if len(line) == 0:
        index += 1
        continue
    
    if len(ports) == index:
        ports.append({})
    for pair in line.split():
        key, value = pair.split(":")
        ports[index][key] = value


valid_keys = {
    "ecl": re.compile(r"amb|blu|brn|gry|grn|hzl|oth"),
    "pid": re.compile(r"^[0-9]{9}$"),
    "eyr": re.compile(r"20(2[0-9]|30)"),
    "hcl": re.compile(r"#[0-9a-f]{6}"),
    "byr": re.compile(r"19[2-9][0-9]|200[0-2]"),
    "iyr": re.compile(r"20(1[0-9]|20)"),
    "hgt": re.compile(r"1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in"),
}


valid = 0
for port in ports:
    if all(key in port and valid_keys[key].match(port[key]) for key in valid_keys):
        print(port["pid"])
        valid += 1

print(valid)



