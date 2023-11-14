import sys

def most_common_bit(data, pos, preference):
    count = [sum(bits) for bits in zip(*data)][pos]
    if count > len(data)/2:
        return True
    elif count == len(data)/2:
        return preference
    else:
        return False

def filter_by_bit(data, pos, bit):
    return [entry for entry in data if entry[pos] == bit]


lines = [[c == "1" for c in list(line.rstrip())] for line in sys.stdin]

filtered = lines
for i in range(len(filtered[0])):
    if len(filtered) == 1:
        break;
    match_bit = most_common_bit(filtered, i, True)
    filtered = filter_by_bit(filtered, i, True);

print filtered
