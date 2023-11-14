import sys

depth = 0
prev = 0

count = 0
for line in sys.stdin:
    prev = depth
    depth = int(line)
    if prev == 0:
        continue

    count += depth > prev

print(count)

