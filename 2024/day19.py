def read_input(path):
    with open(path, "r") as file:
        towels = file.readline().strip().split(', ')
        file.readline()
        designs = [line.strip() for line in file]
        return (towels, designs)


def possible(design, towels, seen, n):
    if len(design) == 0:
        return False
    if design in seen:
        return seen[design]
    if design in towels:
        seen[design] = True
        return True

    # check if design can be broken into multiple towels
    for i in range(1, n):
        if design[:i] in towels and possible(design[i:], towels, seen, n):
            seen[design] = True
            return True

    seen[design] = False
    return False


def part_one(path):
    (towels, designs) = read_input(path)

    n = max(len(towel) for towel in towels) + 1
    seen = {}
    out = 0

    for design in designs:
        if possible(design, towels, seen, n):
            out += 1

    return out


assert part_one('data/19.1.txt') == 6
print('part one:', part_one('data/19.2.txt'))


def count_possible(design, towels, seen, n):
    if design in seen:
        return seen[design]
    if len(design) == 0:
        return 0

    for i in range(1, n):
        substring = design[:i]
        if substring in towels:
            if len(design) == i:
                update_seen(seen, design, 1)
            else:
                update_seen(seen, design, count_possible(design[i:], towels, seen, n))

    if design not in seen:
        seen[design] = 0

    return seen[design]


def update_seen(seen, design, count):
    if design in seen:
        seen[design] += count
    else:
        seen[design] = count


def part_two(path):
    (towels, designs) = read_input(path)

    n = max(len(towel) for towel in towels) + 1
    seen = {}
    out = 0

    for design in designs:
        out += count_possible(design, towels, seen, n)

    return out


assert part_two('data/19.1.txt') == 16
print('part two:', part_two('data/19.2.txt'))
