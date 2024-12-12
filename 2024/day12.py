def read_input(path):
    map = []
    with open(path, 'r') as file:
        for line in file:
            map.append(list(line.strip()))
    return map


def part_one(path):
    map = read_input(path)
    seen = set()

    price = 0

    for i in range(len(map)):
        for j in range(len(map[0])):
            if (i, j) not in seen:
                # area, perimeter, seen
                book_keeping = [0, 0, seen]
                flood(map, i, j, map[i][j], book_keeping)
                price += book_keeping[0] * book_keeping[1]

    return price


def flood(map, i, j, target, books):
    if i < 0 or j < 0 or i >= len(map) or j >= len(map[0]):
        return False
    if map[i][j] != target:
        return False
    if (i, j) in books[2]:
        return True

    books[0] += 1
    books[2].add((i, j))

    left = flood(map, i, j - 1, target, books)
    up = flood(map, i - 1, j, target, books)
    right = flood(map, i, j + 1, target, books)
    down = flood(map, i + 1, j, target, books)

    for result in [left, up, right, down]:
        if not result:
            books[1] += 1

    return True


assert part_one('data/12.1.txt') == 1930
print('part one:', part_one('data/12.2.txt'))
