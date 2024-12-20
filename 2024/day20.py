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


def part_one(path, min_save_to_matter, cheating_time_allowed):
    graph = read_input(path)
    start, = (key for key, val in graph.items() if val == 'S')
    end, = (key for key, val in graph.items() if val == 'E')

    (base_dist, path) = shortest_path(graph, start, end)

    score_from = dict()
    for i, node in enumerate(path):
        score_from[node] = base_dist - i

    out = 0
    for i, cheat_start in enumerate(path):
        for j, cheat_end in enumerate(path[i:]):
            if cheat_start == cheat_end:
                continue
            taxicab = sum(abs(a - b) for a, b in zip(cheat_start, cheat_end))
            if taxicab <= cheating_time_allowed:
                score = i + taxicab + score_from[cheat_end]
                diff = abs(score - base_dist)
                if diff >= min_save_to_matter:
                    out += 1

    return out


assert part_one('data/20.1.txt', 2, 2) == 44
print('part one:', part_one('data/20.2.txt', 100, 2))
assert part_one('data/20.1.txt', 50, 20) == 285
print('part two:', part_one('data/20.2.txt', 100, 20))
