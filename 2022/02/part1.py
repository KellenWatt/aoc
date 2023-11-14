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


lines = [line.rstrip() for line in sys.stdin]
matches = [[normalize(play) for play in l.split(" ")] for l in lines]
scores = [] * len(matches)

scores = [rps(*m) + m[1] + 1 for m in matches]
print(sum(scores))

