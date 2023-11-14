import sys


def normalize(point, offset):
    return (point[0] - offset[0], point[1] - offset[1])

def out_of_bounds(point, corner):
    x, y = point
    return x < 0 or x > corner[0] or y < 0 or y > corner[1]

def draw_line(grid, start, end):
    if start[0] == end[0]:
        if end[1] < start[1]:
            s, e = end[1], start[1]
        else:
            s, e = start[1], end[1]
        for y in range(s, e + 1):
            grid[y][start[0]] = "#"
    elif start[1] == end[1]:
        if end[0] < start[0]:
            s, e = end[0], start[0]
        else:
            s, e = start[0], end[0]
        for x in range(s, e + 1):
            grid[start[1]][x] = "#"
    else:
        raise ValueError("Cannot draw angled lines")

def simulate_granule(grid, pos):
    "returns the next position for the granule"
    occupied = lambda p: grid[p[1]][p[0]] != " "

    for off in [0, -1, 1]:
        dest = (pos[0] + off, pos[1] + 1)
        if out_of_bounds(dest) or not occupied(dest):
            return dest

    return pos



lines = [line.rstrip() for line in sys.stdin]
paths = [[eval("({})".format(pair)) for pair in line.split(" -> ")] for line in lines]

minx = min(min(x for x,y in path) for path in paths)
maxx = max(max(x for x,y in path) for path in paths)
miny = min(min(y for x,y in path) for path in paths)
maxy = max(max(y for x,y in path) for path in paths)

upper_left = (minx, miny)
lower_right = (maxx, maxy)


grid = [[" " for _ in range(maxx - minx)] for _ in range(maxy - miny)]

normal_paths = [[normalize(point, upper_left) for point in path] for path in paths]
entrance = normalize((500, miny), upper_left)

print(normal_paths)

for path in paths:
    for i in range(len(path)-1):
        start, end = path[i], path[i+1]
        draw_line(grid, end, start)

for line in grid:
    print("".join(line))

exit(0)
# formatting done.

granules = []
prev_len = -1
while len(granules) != prev_len:
    prev_len = len(granules)
    pos = entrance
    while True:
        fall = simulate_granule(grid, pos)
        if out_of_bounds(fall):
            # We're falling forever, and the granule shouldn't be added
            break
        if fall == pos:
            # we've come to rest
            grid[fall[1]][fall[0]] = "o"
            granules.append(pos)
            break
        # keep on falling
        pos = fall
            
        







