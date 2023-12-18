import sys

line = [line.rstrip() for line in sys.stdin][0]

floor = 0
for i, c in enumerate(line):
    if c == "(":
        floor += 1
    else:
        floor -= 1
    if floor < 0:
        print(i+1)
        break
