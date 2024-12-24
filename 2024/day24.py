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
    (_wires, instructions) = read_input(path)
    outputs = [i for i in instructions if i[3].startswith('z')]
    outputs.sort(key=lambda a: a[3])

    bad = set()

    for instruction in instructions:
        (a, op, b, target) = instruction
        if target.startswith('z'):
            if target == outputs[-1][3]:
                if op != 'OR':
                    bad.add(instruction)
                    continue
            elif op != 'XOR':
                bad.add(instruction)
                continue
        elif op == 'XOR':
            if not ((a.startswith('x') or a.startswith('y')) and (b.startswith('x') or b.startswith('y'))):
                bad.add(instruction)
                continue
            else:
                matches = [x for x in instructions if x[1] ==
                           'XOR' and (x[0] == target or x[2] == target)]
                if len(matches) == 0:
                    bad.add(instruction)
                    continue
        elif op == 'AND':
            matches = [x for x in instructions if x[1] ==
                       'OR' and (x[0] == target or x[2] == target)]
            if len(matches) == 0:
                bad.add(instruction)
                continue

    bad_wires = [x[3] for x in bad if x[0] != 'x00']
    print(','.join(sorted(bad_wires)))


part_two('data/24.3.txt')

# outputs
# [
#     ('y00', 'XOR', 'x00', 'z00'),
#     ('qvn', 'XOR', 'sgr', 'z01'),
#     ('btn', 'XOR', 'sgv', 'z02'),
#     ('jtc', 'XOR', 'sng', 'z03'),
#     ('gbh', 'XOR', 'whq', 'z04'),
#     ('kbv', 'XOR', 'csv', 'z05'),
#     ('srn', 'XOR', 'fhb', 'z06'),
#     ('ksv', 'XOR', 'cnk', 'z07'),
#     ('kwv', 'OR', 'ctv', 'z08'),  # very sus
#     ('vvr', 'XOR', 'ggf', 'z09'),
#     ('nsm', 'XOR', 'vkt', 'z10'),
#     ('tpf', 'XOR', 'sft', 'z11'),
#     ('kqp', 'XOR', 'hcc', 'z12'),
#     ('gps', 'XOR', 'bkk', 'z13'),
#     ('vng', 'XOR', 'gfs', 'z14'),
#     ('sfc', 'XOR', 'sdv', 'z15'),
#     ('pvv', 'XOR', 'rnq', 'z16'),
#     ('kbg', 'XOR', 'hth', 'z17'),
#     ('kbw', 'XOR', 'jnv', 'z18'),
#     ('npr', 'XOR', 'hfv', 'z19'),
#     ('pkb', 'XOR', 'tbr', 'z20'),
#     ('pkq', 'XOR', 'jsk', 'z21'),
#     ('fhn', 'XOR', 'fvq', 'z22'),
#     ('vvv', 'XOR', 'kmd', 'z23'),
#     ('kpb', 'XOR', 'hkf', 'z24'),
#     ('wwt', 'XOR', 'vbw', 'z25'),
#     ('bhv', 'XOR', 'wnf', 'z26'),
#     ('nct', 'XOR', 'njn', 'z27'),
#     ('y28', 'AND', 'x28', 'z28'),  # very sus
#     ('gwp', 'XOR', 'vst', 'z29'),
#     ('nrr', 'XOR', 'qwg', 'z30'),
#     ('rjb', 'XOR', 'btj', 'z31'),
#     ('msf', 'XOR', 'tkt', 'z32'),
#     ('brn', 'XOR', 'jkv', 'z33'),
#     ('jpb', 'XOR', 'kpm', 'z34'),
#     ('qgs', 'XOR', 'mjn', 'z35'),
#     ('bbk', 'XOR', 'fcn', 'z36'),
#     ('kbk', 'XOR', 'hbw', 'z37'),
#     ('jjj', 'XOR', 'fpw', 'z38'),
#     ('thk', 'AND', 'wnk', 'z39'),  # very sus
#     ('jds', 'XOR', 'gqd', 'z40'),
#     ('qvf', 'XOR', 'jmm', 'z41'),
#     ('nmw', 'XOR', 'jdf', 'z42'),
#     ('ncj', 'XOR', 'gdc', 'z43'),
#     ('tmh', 'XOR', 'qch', 'z44'),
#     ('vpd', 'OR', 'dtb', 'z45')
# ]
