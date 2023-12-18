import sys

line = [line.rstrip() for line in sys.stdin][0]

floor = 0
for c in line:
    if c == "(":
        floor += 1
    else:
        floor -= 1

print(floor)
