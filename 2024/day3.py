import re

MUL_REGEX = r'mul\((\d+),(\d+)\)'


def read_input(path):
    program = ""

    with open(path, 'r') as file:
        for line in file:
            program += line

    return program


def execute(program):
    return sum((int(a) * int(b)) for (a, b) in re.findall(MUL_REGEX, program))


def part_one(path):
    return execute(read_input(path))


assert part_one('data/3.1.txt') == 161
print('part one:', part_one('data/3.2.txt'))


def part_two(path):
    program = read_input(path)
    segments = program.split("don't()")

    parsed = segments[0]
    for i in range(1, len(segments)):
        keep = segments[i].split('do()')
        parsed += ''.join(keep[1:])

    return execute(parsed)


assert part_two('data/3.3.txt') == 48
print('part two:', part_two('data/3.2.txt'))
