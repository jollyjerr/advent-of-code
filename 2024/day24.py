def read_input(path):
    wires = dict()
    instructions = []
    with open(path, "r") as file:
        for line in file:
            if ':' in line:
                [k, v] = line.strip().split(': ')
                wires[k] = int(v)
            elif '->' in line:
                parts = line.strip().split(' ')
                instructions.append((parts[0], parts[1], parts[2], parts[4]))
    return (wires, instructions)


operations = {
    'AND': lambda a, b: 1 if a and b else 0,
    'OR': lambda a, b: 1 if a or b else 0,
    'XOR': lambda a, b: 1 if a != b else 0
}


def part_one(path):
    (wires, instructions) = read_input(path)

    while instructions:
        instruction = instructions.pop(0)
        (a, op, b, target) = instruction
        if a in wires and b in wires:
            wires[target] = operations[op](wires[a], wires[b])
        else:
            instructions.append(instruction)

    zeds = sorted([(k, v) for k, v in wires.items() if k.startswith('z')], reverse=True)
    return int(''.join([str(v) for (_, v) in zeds]), 2)


assert part_one('data/24.1.txt') == 4
assert part_one('data/24.2.txt') == 2024
print('part one:', part_one('data/24.3.txt'))


def part_two(path):
    (wires, instructions) = read_input(path)
