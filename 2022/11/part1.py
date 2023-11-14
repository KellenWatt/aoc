from typing import Callable
import sys
import re


class Monkey:
    items: list[int]
    inspector: Callable[[int], int]
    pass_to: Callable[[int], tuple[int, int]]
    items_seen: int
    
    def from_block(text):
        lines = text.split("\n")
        
        items = re.search("Starting items: (\d+(, \d+)*)", lines[1]).group(1).split(", ")
        items = [int(item) for item in items]
        
        details = re.search("Operation: new = (old (?:\+|\*) (?:\d+|old))", lines[2])
        op = eval("lambda old: " + details.group(1))
        
        test_div = re.search("divisible by (\d+)", lines[3]).group(1)
        pass_test = lambda worry: (worry % int(test_div)) == 0

        true_monkey = int(re.search("(\d+)", lines[4]).group(1))
        false_monkey = int(re.search("(\d+)", lines[5]).group(1))
        
        return Monkey(items, op, pass_test, true_monkey, false_monkey)


    def __init__(self, items, op, pass_test, true_monkey, false_monkey):
        self.items = items
        self.inspector = op
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.pass_to = lambda item: true_monkey if pass_test(item) else false_monkey
        self.items_seen = 0

    def inspect_next(self):
        item = self.items.pop(0)
        item = self.inspector(item) // 3
        self.items_seen += 1
        return (item, self.pass_to(item))

    def has_items(self):
        return len(self.items) > 0

    def catch(self, item):
        self.items.append(item)





lines = [line.rstrip() for line in sys.stdin]
monkey_texts = "\n".join(lines).split("\n\n")
monkeys = [Monkey.from_block(m) for m in monkey_texts]


for i, m in enumerate(monkeys):
    print("{}:: true: {}, false: {}".format(i, m.true_monkey, m.false_monkey))
    print("    items: " + ", ".join([str(i) for i in m.items]))

for _ in range(20):
    for m in range(len(monkeys)):
        while monkeys[m].has_items():
            item, target = monkeys[m].inspect_next()
            monkeys[target].catch(item)

ms = sorted(monkeys, key = lambda m: m.items_seen)[-2:]
print(ms[0].items_seen * ms[1].items_seen)


