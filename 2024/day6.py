def read_input(path):
    map = []
    start = ()

    with open(path, 'r') as file:
        for (i, line) in enumerate(file):
            row = line.strip()
            map.append(list(row))
            if '^' in row:
                j = row.index('^')
                start = (i, j)

    return (map, start)


def walk(map, start):
    # left, up, right, down
    steps = [(0, -1), (-1, 0), (0, 1), (1, 0)]
    turns = [1, 2, 3, 0]
    direction = 1

    location = start
    map[location[0]][location[1]] = 1

    unique_blocks = 1
    loop_found = False

    while True:
        (delta_x, delta_y) = steps[direction]
        new_location = (location[0] + delta_x, location[1] + delta_y)

        if new_location[0] < 0 or new_location[0] >= len(map):
            break
        if new_location[1] < 0 or new_location[1] >= len(map[0]):
            break

        next_block = map[new_location[0]][new_location[1]]
        if next_block == '.' or isinstance(next_block, int):
            location = new_location
            if next_block == '.':
                unique_blocks += 1
                map[new_location[0]][new_location[1]] = 1
            elif isinstance(next_block, int):
                if next_block > 3:
                    loop_found = True
                    break
                map[new_location[0]][new_location[1]] += 1
        else:
            direction = turns[direction]

    return (unique_blocks, loop_found)


def part_one(path):
    (map, start) = read_input(path)
    return walk(map, start)[0]


assert part_one('data/6.1.txt') == 41
print('part one:', part_one('data/6.2.txt'))


def part_two(path):
    (map, start) = read_input(path)

    out = 0
    for i in range(len(map)):
        for j in range(len(map[0])):
            if map[i][j] == '.':
                copy = [row[:] for row in map]
                copy[i][j] = '#'
                (_, loop_found) = walk(copy, start)
                if loop_found:
                    out += 1

    return out


assert part_two('data/6.1.txt') == 6
print('part two:', part_two('data/6.2.txt'))
