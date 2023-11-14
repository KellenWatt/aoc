import sys

lines = [line.rstrip() for line in sys.stdin]

elves = [[]]
for l in lines:
    if not l:
        elves.append([])
        continue

    elves[-1].append(int(l))

count_by_elf = [sum(e) for e in elves]

print(sum(sorted(count_by_elf, reverse = True)[:3]))
