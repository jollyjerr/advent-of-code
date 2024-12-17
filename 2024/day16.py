from heapq import heappush, heappop


def read_input(path):
    nodes = {(i, j): col for i, row in enumerate(open(path, "r"))
             for j, col in enumerate(row.strip()) if col != '#'}
    start, = (cords for cords in nodes if nodes[cords] == 'S')
    end, = (cords for cords in nodes if nodes[cords] == 'E')
    return (nodes, start, end)


# west, north, east, south
directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]
possible_directions = [[3, 0, 1], [0, 1, 2], [1, 2, 3], [2, 3, 0]]


def part_one(path):
    (nodes, start, end) = read_input(path)

    costs = {}  # (row, column, direction) -> min_cost
    queue = []  # (min_cost, (row, column, direction))

    start_state = (start[0], start[1], 2)
    costs[start_state] = 0
    heappush(queue, (0, start_state))

    while queue:
        (min_cost, (row, col, direction)) = heappop(queue)

        if (row, col) in costs:
            continue
        elif (row, col) == end:
            return costs[(row, col, direction)]

        for idx in possible_directions[direction]:
            weight = 1 if idx == direction else 1001
            (dr, dc) = directions[idx]
            nr, nc = row + dr, col + dc

            if (nr, nc) in nodes:
                next_cost = min_cost + weight
                next_state = (nr, nc, idx)

                if next_cost < costs.get(next_state, float('inf')):
                    costs[next_state] = next_cost
                    heappush(queue, (next_cost, next_state))

    return 0


assert part_one('data/16.1.txt') == 7036
assert part_one('data/16.2.txt') == 11048
print('part one: ', part_one('data/16.3.txt'))


# I literally hate this
def part_two(path):
    (nodes, start, end) = read_input(path)

    costs = {}  # (row, column, direction) -> min_cost
    paths = {}  # (row, column, direction) -> set of nodes in path
    queue = []  # (min_cost, (row, column, direction))

    start_state = (start[0], start[1], 2)
    costs[start_state] = 0
    paths[start_state] = {(start[0], start[1])}
    heappush(queue, (0, start_state))

    min_total_cost = float('inf')
    shortest_paths = []

    while queue:
        (min_cost, (row, col, direction)) = heappop(queue)

        if (row, col) == end:
            if min_cost < min_total_cost:
                min_total_cost = min_cost
                shortest_paths = [paths[(row, col, direction)]]
            elif min_cost == min_total_cost:
                shortest_paths.append(paths[(row, col, direction)])
            continue

        for idx in possible_directions[direction]:
            weight = 1 if idx == direction else 1001
            (dr, dc) = directions[idx]
            nr, nc = row + dr, col + dc

            if (nr, nc) in nodes:
                next_cost = min_cost + weight
                next_state = (nr, nc, idx)

                if next_cost <= costs.get(next_state, float('inf')):
                    if next_cost < costs.get(next_state, float('inf')):
                        costs[next_state] = next_cost
                        next_path = paths[(row, col, direction)].copy()
                        next_path.add((nr, nc))
                        paths[next_state] = next_path
                    else:
                        existing_path = paths.get(next_state, set())
                        new_path = paths[(row, col, direction)].copy()
                        new_path.add((nr, nc))
                        paths[next_state] = existing_path.union(new_path)

                    heappush(queue, (next_cost, next_state))

    seats = set()
    for path in shortest_paths:
        seats.update(path)

    return len(seats)


assert part_two('data/16.1.txt') == 45
assert part_two('data/16.2.txt') == 64
print('part two:', part_two('data/16.3.txt'))
