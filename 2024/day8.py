from collections import defaultdict


def read_input(path):
    map = []
    with open(path, 'r') as file:
        for line in file:
            map.append(list(line.strip()))
    return map


def get_antennas(map):
    antennas = defaultdict(set)

    for i in range(len(map)):
        for j in range(len(map[i])):
            if map[i][j] != '.':
                antennas[map[i][j]].add((i, j))

    return antennas


def get_next_points_out_on_vec(lower, higher, delta_x, delta_y):
    return [
        (lower[0] - delta_y, lower[1] - delta_x),
        (higher[0] + delta_y, higher[1] + delta_x)
    ]


def in_bounds(point, map):
    (y, x) = point
    return x > -1 and x < len(map) and y > -1 and y < len(map[0])


def part_one(path):
    map = read_input(path)
    antennas = get_antennas(map)
    antinodes = set()

    for key in antennas:
        locations = antennas[key]
        while locations:
            a = locations.pop()
            for b in locations:
                lower = a if a[0] < b[0] else b
                higher = a if lower == b else b
                delta_y = higher[0] - lower[0]
                delta_x = higher[1] - lower[1]
                for point in get_next_points_out_on_vec(lower, higher, delta_x, delta_y):
                    if in_bounds(point, map):
                        antinodes.add(point)

    return len(antinodes)


assert part_one('data/8.1.txt') == 14
print('part one:', part_one('data/8.2.txt'))


def part_two(path):
    map = read_input(path)
    antennas = get_antennas(map)
    antinodes = set()

    for key in antennas:
        locations = antennas[key]
        while locations:
            a = locations.pop()
            for b in locations:
                lower = a if a[0] < b[0] else b
                higher = a if lower == b else b
                delta_y = higher[0] - lower[0]
                delta_x = higher[1] - lower[1]

                antinodes.add(lower)
                antinodes.add(higher)

                while True:
                    [lower_p, higher_p] = get_next_points_out_on_vec(
                        lower, higher, delta_x, delta_y
                    )
                    if not in_bounds(lower_p, map) and not in_bounds(higher_p, map):
                        break
                    if in_bounds(lower_p, map):
                        lower = lower_p
                        antinodes.add(lower_p)
                    if in_bounds(higher_p, map):
                        higher = higher_p
                        antinodes.add(higher_p)

    return len(antinodes)


assert part_two('data/8.1.txt') == 34
print('part two:', part_two('data/8.2.txt'))
