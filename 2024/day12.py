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
    if not is_target(map, i, j, target):
        return False
    if (i, j) in books[2]:
        return True

    books[0] += 1
    books[2].add((i, j))

    left = flood(map, i, j - 1, target, books)
    up = flood(map, i - 1, j, target, books)
    right = flood(map, i, j + 1, target, books)
    down = flood(map, i + 1, j, target, books)

    results = [left, up, right, down]

    if len(books) > 3:
        up_left = is_target(map, i - 1, j - 1, target)
        up_right = is_target(map, i - 1, j + 1, target)
        down_left = is_target(map, i + 1, j - 1, target)
        down_right = is_target(map, i + 1, j + 1, target)

        corners = 0

        if not left and not up:
            corners += 1
        if not up and not right:
            corners += 1
        if not right and not down:
            corners += 1
        if not down and not left:
            corners += 1
        if left and up and (not up_left):
            corners += 1
        if up and right and (not up_right):
            corners += 1
        if right and down and (not down_right):
            corners += 1
        if down and left and (not down_left):
            corners += 1

        books[3] += corners

    for result in results:
        if not result:
            books[1] += 1

    return True


def is_target(map, i, j, target):
    if i < 0 or j < 0 or i >= len(map) or j >= len(map[0]):
        return False
    if map[i][j] != target:
        return False
    return True


assert part_one('data/12.1.txt') == 1930
print('part one:', part_one('data/12.2.txt'))


def part_two(path):
    map = read_input(path)
    seen = set()

    price = 0

    for i in range(len(map)):
        for j in range(len(map[0])):
            if (i, j) not in seen:
                # area, perimeter, seen, corners
                book_keeping = [0, 0, seen, 0]
                flood(map, i, j, map[i][j], book_keeping)
                price += book_keeping[0] * book_keeping[3]

    return price


assert part_two('data/12.1.txt') == 1206
print('part two:', part_two('data/12.2.txt'))
