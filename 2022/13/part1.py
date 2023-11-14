import sys
lines = [line.rstrip() for line in sys.stdin]
pairs = "\n".join(lines).split("\n\n")
pairs = [[eval(l) for l in p.split("\n")] for p in pairs]


def tap_result(res):
    if res == -1:
        print("valid")
    if res == 1:
        print("invalid")
    if res == 0:
        print("undecided")
    return res

# -1 is valid, 0 is undecided, 1 is invalid
def compair(left, right):
    print("comparing: {} vs {}".format(left, right))
    if type(left) == type(right):
        if type(left) == type(0):
            if left < right:
                return -1
            elif left > right:
                return 1
            else:
                return 0
        elif type(left) == type([]):
            iters = min([len(left), len(right)])
            for i in range(iters):
                res = compair(left[i], right[i])
                if res != 0:
                    return res
            # the lists were equal up to the end, now compare length.
            if len(left) < len(right):
                return -1
            elif len(left) > len(right):
                return 1
            else: 
                return 0
    else:
        if type(left) == type(0):
            return compair([left], right)
        else:
            return compair(left, [right])

matches = [i+1 for i, p in enumerate(pairs) if tap_result(compair(*p)) == -1]
print(matches)
print(sum(matches))
