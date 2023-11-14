import sys

depths = [int(line) for line in sys.stdin]

windows = [sum(depths[i:i+3]) for i in range(len(depths) - 2)]

print(len([1 for i in range(len(windows)-1) if windows[i] < windows[i+1]]))
