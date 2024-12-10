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


DEBUG = False


def part_two(path):
    data = read_input(path)
    blocks = [(key, len(list(group))) for key, group in groupby(data)]

    if DEBUG:
        for_print = [n for n, count in blocks for _ in range(count)]
        print(''.join([f'{x}' if x >= 0 else '.' for x in for_print]))

    seen = set()
    pointer = len(blocks) - 1
    while True:
        # (id, size) with negative id meaning gap
        check = blocks[pointer]
        if check[0] < 0 or check in seen:
            pointer -= 1
            if pointer > 0:
                continue
            else:
                break
        seen.add(check)
        true_len = len(str(check[0])) * check[1]
        for j in range(0, pointer):
            spot = blocks[j]
            if spot[0] >= 0 or spot[1] < true_len:
                continue
            blocks[j] = (blocks[j][0], blocks[j][1] - true_len)
            blocks[pointer] = (-1, check[1])
            blocks.insert(j, check)
            if DEBUG:
                for_print = [n for n, count in blocks for _ in range(count)]
                print(''.join([f'{x}' if x >= 0 else '.' for x in for_print]))
            break

    expanded = [n for n, count in blocks for _ in range(count)]

    return sum([i * v for (i, v) in enumerate(expanded) if v >= 0])


assert part_two('data/9.1.txt') == 2858
# print(part_two('data/9.3.txt'))
print('part two', part_two('data/9.2.txt'))

# 13488487148395 - too high
# 6937090627391 - too high
