def read_input(path):
    map = []
    with open(path, 'r') as file:
        for line in file:
            map.append([int(x) for x in list(line.strip())])
    return map


def search(map, i, j, target, counter):
    results = set()

    if i < 0 or j < 0 or i >= len(map) or j >= len(map[0]):
        return results
    if map[i][j] != target:
        return results
    if target == 9:
        counter[0] += 1
        results.add((i, j))
        return results

    next_target = target + 1

    left = search(map, i, j - 1, next_target, counter)
    up = search(map, i - 1, j, next_target, counter)
    right = search(map, i, j + 1, next_target, counter)
    down = search(map, i + 1, j, next_target, counter)

    for direction in [left, up, right, down]:
        for entry in direction:
            results.add(entry)

    return results


def part_one(path, counter=[0]):
    map = read_input(path)

    out = 0
    for i in range(len(map)):
        for j in range(len(map[0])):
            out += len(search(map, i, j, 0, counter))

    return out


assert part_one('data/10.1.txt') == 1
assert part_one('data/10.2.txt') == 36
print('part one:', part_one('data/10.3.txt'))


def part_two(path):
    counter = [0]
    part_one(path, counter)
    return counter[0]


assert part_two('data/10.2.txt') == 81
print('part two:', part_two('data/10.3.txt'))
