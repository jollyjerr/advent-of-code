from collections import defaultdict


def read_input(path):
    rules = defaultdict(list)
    updates = []

    with open(path, 'r') as file:
        for line in file:
            if '|' in line:
                [before, after] = line.strip().split('|')
                rules[int(before)].append(int(after))
            elif ',' in line:
                updates.append([int(c) for c in line.strip().split(',')])

    return (rules, updates)


def part_one(path):
    (rules, updates) = read_input(path)

    out = 0
    for update in updates:
        correct = True
        for i in range(len(update)):
            rule = rules[update[i]]
            if not all([val in rule for val in update[i + 1:]]):
                correct = False
        if correct:
            out += update[(len(update) - 1) // 2]

    return out


assert part_one('data/5.1.txt') == 143
print('part one:', part_one('data/5.2.txt'))


def part_two(path):
    (rules, updates) = read_input(path)

    out = 0
    for update in updates:
        for i in range(len(update)):
            rule = rules[update[i]]
            if not all([val in rule for val in update[i + 1:]]):
                sorted = []
                unsorted = set(update)
                while unsorted:
                    for value in unsorted:
                        if all(other in rules[value] for other in unsorted if other != value):
                            sorted.append(value)
                            unsorted.remove(value)
                            break
                out += sorted[(len(sorted) - 1) // 2]
                break

    return out


assert part_two('data/5.1.txt') == 123
print('part two:', part_two('data/5.2.txt'))
