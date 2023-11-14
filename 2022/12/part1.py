import sys

def elevate(c):
    if c == "S":
        return ord("a")
    elif c == "E":
        return ord("z")
    else:
        return ord(c)

def height(grid, pos):
    return grid[pos[1]][pos[0]]


def nexts(grid, pos):
    h = height(grid, pos)

    out = []
    if pos[0] > 0:
        out.append((pos[0]-1, pos[1]))
    if pos[0] < len(grid[0]) - 1:
        out.append((pos[0]+1, pos[1]))
    if pos[1] > 0:
        out.append((pos[0], pos[1]-1))
    if pos[1] < len(grid) - 1:
        out.append((pos[0], pos[1]+1))

    return [p for p in out if h - height(grid, p) <= 1]

def viz(grid, seen):
    viz_map = [r.copy() for r in grid]
    for s in seen:
        viz_map[s[1]][s[0]] = ord(".")
    return viz_map

lines = [line.rstrip() for line in sys.stdin]

for y, l in enumerate(lines):
    try:
        sx = l.index("S")
        start = (sx, y)
    except ValueError:
        pass

    try: 
        ex = l.index("E")
        end = (ex, y)
    except ValueError:
        pass

terrain = [[elevate(c) for c in list(line)] for line in lines]


class Node:
    pos: tuple[int, int]
    prev: "Node"
    dist: int

    def __init__(self, pos, prev = None):
        self.pos = pos
        self.prev = prev
        if prev is None:
            self.dist = 0
        else:
            self.dist = prev.dist + 1

seen = set()
queue = [Node(end)]

dest = None

while not len(queue) == 0:
    node = queue.pop(0)
    if node.pos in seen:
        continue

    seen.add(node.pos)
    if node.pos == start:
        dest = node
        break

    for n in nexts(terrain, node.pos):
        if n in seen:
            continue
        queue.append(Node(n, node))

if dest is None:
    print("No path found!")
else:
    print(dest.dist)

for row in viz(terrain, seen):
    print("".join([chr(c) for c in row]))
