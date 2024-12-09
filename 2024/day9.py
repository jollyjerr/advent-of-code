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
                        out.append('.')
                if i % 2 == 0:
                    file_id += 1
    return out


def part_one(path):
    data = read_input(path)
    queue = [x for x in data[::-1] if x != '.']
    n = len(queue)

    result = [queue.pop(0) if data[i] == '.' else data[i] for i in range(n)]

    return sum([i * v for (i, v) in enumerate(result) if v != '.'])


assert part_one('data/9.1.txt') == 1928
print('part one:', part_one('data/9.2.txt'))
