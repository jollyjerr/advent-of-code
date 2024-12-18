from heapq import heappush, heappop


def read_input(path):
    out = []
    with open(path, "r") as file:
        for line in file:
            [x, y] = line.strip().split(",")
            out.append((int(y), int(x)))
    return out


directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]


def part_one(path, n, bytes):
    map = [['.'] * (n + 1) for _ in range(n + 1)]

    memory = read_input(path)
    for i in range(bytes):
        (row, col) = memory.pop(0)
        map[row][col] = '#'

    costs = {(0, 0): 0}
    queue = []
    heappush(queue, (0, (0, 0)))

    while queue:
        min_cost, (row, col) = heappop(queue)

        if (col, row) == (n, n):
            return min_cost

        if min_cost > costs.get((row, col), float('inf')):
            continue

        for (dr, dc) in directions:
            nr, nc = row + dr, col + dc

            if 0 <= nr < (n + 1) and 0 <= nc < (n + 1) and map[nr][nc] != '#':
                next_cost = min_cost + 1
                if next_cost < costs.get((nr, nc), float('inf')):
                    costs[(nr, nc)] = next_cost
                    heappush(queue, (next_cost, (nr, nc)))

    return -1


assert part_one('data/18.1.txt', 6, 12) == 22
print('part one:', part_one('data/18.2.txt', 70, 1024))


def part_two(path, n):
    memory = read_input(path)

    for i in range(len(memory)):
        # this off by one error got me good
        if part_one(path, n, i + 1) == -1:
            mem = memory[i]
            return f'{mem[1]},{mem[0]}'
    print('all valid')


assert part_two('data/18.1.txt', 6) == '6,1'
print('part two', part_two('data/18.2.txt', 70))
