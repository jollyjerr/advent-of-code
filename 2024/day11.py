from collections import defaultdict


def read_input(path):
    stones = defaultdict(lambda: 0)
    with open(path, 'r') as file:
        for line in file:
            for stone in [int(x) for x in line.strip().split(' ')]:
                stones[stone] += 1
    return stones


def part_one(path, blinks):
    stones = read_input(path)

    for i in range(blinks):
        results = defaultdict(lambda: 0)
        for key, value in stones.items():
            if key == 0:
                results[1] += value
                continue

            strk = str(key)
            if len(strk) % 2 == 0:
                mid = len(strk) // 2
                left, right = int(strk[:mid]), int(strk[mid:])
                results[left] += value
                results[right] += value
            else:
                results[key * 2024] += value
        stones = results

    return sum(stones.values())


assert part_one('data/11.3.txt', 1) == 7
assert part_one('data/11.1.txt', 25) == 55312
print('part one:', part_one('data/11.2.txt', 25))
print('part two:', part_one('data/11.2.txt', 75))
