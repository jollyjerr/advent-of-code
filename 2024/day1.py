def read_lists(path):
    with open(path, 'r') as file:
        lines = file.readlines()

    list_1 = []
    list_2 = []

    for line in lines:
        data = line.split('   ')
        list_1.append(int(data[0]))
        list_2.append(int(data[1]))

    return (list_1, list_2)


def part_one(path):
    (list_1, list_2) = read_lists(path)

    list_1.sort()
    list_2.sort()

    out = 0
    for (a, b) in zip(list_1, list_2):
        out += (abs(a - b))

    return out


assert part_one('data/1.1.txt') == 11
print('part one:', part_one('data/1.2.txt'))


def part_two(path):
    (list_1, list_2) = read_lists(path)

    out = 0
    for i in list_1:
        out += (i * list_2.count(i))

    return out


assert part_two('data/1.1.txt') == 31
print('part two:', part_two('data/1.2.txt'))
