def read_input(path):
    with open(path, "r") as file:
        locks = []
        keys = []

        while True:
            grid = []
            for i in range(7):
                grid.append(file.readline().strip())
            counts = []
            for i in range(len(grid[0])):
                counts.append(-1)
                for j in range(len(grid)):
                    counts[i] += 1 if grid[j][i] == '#' else 0
            if '#' in grid[0]:
                locks.append(counts)
            else:
                keys.append(counts)
            if file.readline() == '':
                return (locks, keys)


def part_one(path):
    (locks, keys) = read_input(path)

    out = 0

    for lock in locks:
        for key in keys:
            valid = True
            for i in range(len(key)):
                if lock[i] + key[i] >= 6:
                    valid = False
            if valid:
                out += 1

    return out


assert part_one('data/25.1.txt') == 3
print('part one:', part_one('data/25.2.txt'))
