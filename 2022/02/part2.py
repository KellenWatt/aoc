import sys


def rps(op, me):
    if op == (me - 1)%3:
        return 6
    elif op == (me+1) % 3:
        return 0
    else:
        return 3

def normalize(play):
    if play < "D":
        return ord(play) - ord("A")
    else:
        return ord(play) - ord("X")

def get_pair(op, outcome):
    if outcome == 0:
        return (op-1) % 3
    elif outcome == 2:
        return (op+1) % 3
    else:
        return op


lines = [line.rstrip() for line in sys.stdin]
matches = [[normalize(play) for play in l.split()] for l in lines]

scores = [3*out + get_pair(op,out) + 1 for op, out in matches ]

print(sum(scores))

