def read_input(path):
    word_search = []

    with open(path, 'r') as file:
        for line in file:
            word_search.append(list(line.strip()))

    return word_search


def vecs(i, j):
    return [
        [(i, j + 1), (i, j + 2), (i, j + 3)],  # right
        [(i, j - 1), (i, j - 2), (i, j - 3)],  # left
        [(i + 1, j), (i + 2, j), (i + 3, j)],  # down
        [(i - 1, j), (i - 2, j), (i - 3, j)],  # up
        [(i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],  # down right
        [(i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],  # up left
        [(i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],  # down left
        [(i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],  # up right
    ]


def safe_get(maybe_list, index, fallback):
    if type(maybe_list) is list:
        if index >= 0 and index < len(maybe_list):
            return maybe_list[index]
    return fallback


def part_one(path):
    word_search = read_input(path)

    out = 0

    for i in range(len(word_search)):
        for j in range(len(word_search[i])):
            if word_search[i][j] == 'X':
                indexes = vecs(i, j)
                for vec in indexes:
                    word = ''.join([safe_get(safe_get(word_search, k, []), l, '.')
                                   for (k, l) in vec])
                    if word == 'MAS':
                        out += 1

    return out


assert part_one('data/4.1.txt') == 18
print('part one:', part_one('data/4.2.txt'))


def vecs_two(i, j):
    return [
        [(i - 1, j - 1), (i + 1, j + 1)],
        [(i - 1, j + 1), (i + 1, j - 1)]
    ]


def part_two(path):
    word_search = read_input(path)

    out = 0

    for i in range(len(word_search)):
        for j in range(len(word_search[i])):
            if word_search[i][j] == 'A':
                indexes = vecs_two(i, j)
                hits = 0
                for vec in indexes:
                    word = ''.join([safe_get(safe_get(word_search, k, []), l, '.')
                                   for (k, l) in vec])
                    if word == 'MS' or word == 'SM':
                        hits += 1
                if hits == 2:
                    out += 1

    return out


assert part_two('data/4.1.txt') == 9
print('part two:', part_two('data/4.2.txt'))
