import sys

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


valid_keys = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
valid = 0
for port in ports:
    if all(key in port for key in valid_keys):
        valid += 1

print(valid)



