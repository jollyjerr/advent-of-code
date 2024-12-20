from heapq import heappush, heappop


def read_input(path):
    return {(i, j): c for i, r in enumerate(open(path, "r")) for j, c in enumerate(r.strip()) if c != "#"}


directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]


def shortest_path(graph, start, end):
    costs = {start: 0}
    queue = []
    heappush(queue, (0, start))

    while queue:
        min_cost, (row, col) = heappop(queue)
        if (row, col) == end:
            return (min_cost, [key for key, _ in costs.items()])
        elif min_cost > costs.get((row, col), float('inf')):
            continue
        else:
            for (dr, dc) in directions:
                nr, nc = row + dr, col + dc
                if (nr, nc) in graph:
                    next_cost = min_cost + 1
                    if next_cost < costs.get((nr, nc), float('inf')):
                        costs[(nr, nc)] = next_cost
                        heappush(queue, (next_cost, (nr, nc)))

    return (float('inf'), [])


def end_positions(graph, start):
    (row, col) = start
    return [point for point in [
        (row, col - 2),  # left
        (row - 1, col - 1),  # top left corner
        (row - 2, col),  # top
        (row - 1, col + 1),  # top right corner
        (row, col + 2),  # right
        (row + 1, col + 1),  # bottom right corner
        (row + 2, col),  # bottom
        (row + 1, col - 1)  # bottom left corner
    ] if point in graph]


def part_one(path, min_save_to_matter):
    graph = read_input(path)
    start, = (key for key, val in graph.items() if val == 'S')
    end, = (key for key, val in graph.items() if val == 'E')
    position_to_best_time = dict()  # (i, j) => int
    all_scores = dict()  # ((i, j), (x, y)) => int

    (base_cost, full_route) = shortest_path(graph, start, end)

    for i, node in enumerate(full_route):
        position_to_best_time[node] = base_cost - i

    for i, start_position in enumerate(full_route):
        for end_position in end_positions(graph, start_position):
            cost_from_end = position_to_best_time[end_position]
            all_scores[(start_position, end_position)] = i + 2 + cost_from_end

    out = 0
    winners = [(k, abs(v - base_cost)) for (k, v) in all_scores.items() if v < base_cost]
    for (_k, v) in winners:
        if v >= min_save_to_matter:
            out += 1

    return out


assert part_one('data/20.1.txt', 2) == 44
print('part one:', part_one('data/20.2.txt', 100))
