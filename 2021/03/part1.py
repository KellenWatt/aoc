import sys


lines = [[c == "1" for c in list(line.rstrip())] for line in sys.stdin]
counts = [sum(pos) for pos in zip(*lines)]
print(counts)

gamma = ["1" if count > 500 else "0" for count in counts]
epsilon = ["0" if bit == "1" else "1" for bit in gamma]
print(gamma)
print(epsilon)
gamma = int("".join(gamma), 2)
epsilon = int("".join(epsilon), 2)
print(gamma)
print(epsilon)

print(gamma * epsilon)
