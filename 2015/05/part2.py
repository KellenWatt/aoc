import sys
import re

double_pair = re.compile("(..).*\\1")
mirror = re.compile("(.).\\1")

lines = [line.rstrip() for line in sys.stdin]

goods = []
for line in lines:
    dub = not not double_pair.search(line)
    mir = not not mirror.search(line)
    #  print(line)
    #  print(f"double: {dub}; mirrored: {mir}")
    if mir and dub:
        #  print("good word")
        goods.append(line)

print(len(goods))
