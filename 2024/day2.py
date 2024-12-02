def read_input(path):
    reports = []

    with open(path, 'r') as file:
        for line in file:
            report = [int(i) for i in line.split(' ')]
            reports.append(report)

    return reports


def safe(r):
    safe_ascending = all(r[i] < r[i + 1] and r[i] >= r[i + 1] - 3 for i in range(len(r) - 1))
    safe_descending = all(r[i] > r[i + 1] and r[i] <= r[i + 1] + 3 for i in range(len(r) - 1))
    return safe_ascending or safe_descending


def part_one(path):
    reports = read_input(path)

    out = 0

    for r in reports:
        if safe(r):
            out += 1

    return out


assert part_one('data/2.1.txt') == 2
print('part one:', part_one('data/2.2.txt'))


def part_two(path):
    reports = read_input(path)

    out = 0

    for r in reports:
        if safe(r) or any([safe(r[:i] + r[i + 1:]) for i in range(len(r))]):
            out += 1

    return out


assert part_two('data/2.1.txt') == 4
print('part two:', part_two('data/2.2.txt'))
