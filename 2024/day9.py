from itertools import groupby


def read_input(path):
    out = []
    with open(path, 'r') as file:
        for line in file:
            file_id = 0
            for (i, instruction) in enumerate([int(x) for x in list(line.strip())]):
                for _ in range(instruction):
                    if i % 2 == 0:
                        out.append(file_id)
                    else:
                        out.append(-file_id)
                if i % 2 == 0:
                    file_id += 1
    return out


def part_one(path):
    data = read_input(path)
    queue = [x for x in data[::-1] if x >= 0]
    n = len(queue)

    result = [queue.pop(0) if data[i] < 0 else data[i] for i in range(n)]

    return sum([i * v for (i, v) in enumerate(result) if v >= 0])


assert part_one('data/9.1.txt') == 1928
print('part one:', part_one('data/9.2.txt'))


def part_two(path):
    data = read_input(path)
    blocks = [(key, len(list(group))) for key, group in groupby(data)]
    queue = [x for x in blocks[::-1] if x[0] >= 0]

    for i in range(len(queue)):
        file = queue[i]
        location = blocks.index(file)
        for j in range(0, location):
            block = blocks[j]
            if block[0] < 0 and block[1] >= file[1]:
                blocks[j] = (block[0], block[1] - file[1])
                blocks[location] = (-1, file[1])
                blocks.insert(j, file)
                break

    expanded = [n for n, count in blocks for _ in range(count)]

    return sum([i * v for (i, v) in enumerate(expanded) if v >= 0])


# debug sesh lol
assert part_two('data/9.6.txt') == 2910
assert part_two('data/9.1.txt') == 2858
assert part_two('data/9.3.txt') == 813
assert part_two('data/9.4.txt') == 4
assert part_two('data/9.5.txt') == 132
assert part_two('data/9.7.txt') == 169

real_result = part_two('data/9.2.txt')
assert real_result < 6937090627391
print('part two', part_two('data/9.2.txt'))
