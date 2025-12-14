def part_one(path):
    with open(path, 'r') as file:
        lines = file.read().strip().split('\n')

    out = 0
    for line in lines[30:]:
        dims, quantities = line.split(': ')

        quantities = [int(p) for p in quantities.split(' ')]
        w, h = [int(c) for c in dims.split('x')]

        area = w * h
        quantity = sum(quantities)

        # hardcoded 3*3 shape size for nice input
        if area >= quantity * 3 * 3:
            out += 1

    return out


print('pt one', part_one('data/12.2.txt'))
