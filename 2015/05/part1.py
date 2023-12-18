import sys
import re

three_vowels = re.compile("[aeiou].*[aeiou].*[aeiou]")
double_letter = re.compile("(.)\\1")
rejection = re.compile("ab|cd|pq|xy")

lines = [line.rstrip() for line in sys.stdin]

goods = []
for line in lines:
    tv = not not three_vowels.search(line)
    dub = not not double_letter.search(line)
    bad = not not rejection.search(line)
    if tv and dub and not bad:
        goods.append(line)

print(len(goods))
