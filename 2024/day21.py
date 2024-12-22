def read_input(path):
    return [line.strip() for line in open(path, "r")]


numpad = {'7': (0, 0), '8': (1, 0), '9': (2, 0), '4': (0, 1), '5': (1, 1), '6': (
    2, 1), '1': (0, 2), '2': (1, 2), '3': (2, 2), ' ': (0, 3), '0': (1, 3), 'A': (2, 3)}
dirpad = {' ': (0, 0), '^': (1, 0), 'A': (2, 0), '<': (0, 1), 'v': (1, 1), '>': (2, 1)}


def fewest_presses(layer, target, user_moves):
    return sum(user_moves[(layer, ki, kf)] for ki, kf in zip('A' + target, target))


def solve(code, num_keyboards):
    user_moves = {(0, ki, kf): 1 for ki in dirpad for kf in dirpad}

    for layer in range(1, num_keyboards + 1):
        board = numpad if layer == num_keyboards else dirpad

        # fill in DP table of fewest presses by layer
        for ki, (xi, yi) in board.items():
            for kf, (xf, yf) in board.items():
                lr = ('>' if xf > xi else '<') * abs(xf - xi)
                ud = ('^' if yf < yi else 'v') * abs(yf - yi)

                lr_first = float('inf')
                if (xf, yi) != board[' ']:
                    lr_first = fewest_presses(layer - 1, lr + ud + 'A', user_moves)

                ud_first = float('inf')
                if (xi, yf) != board[' ']:
                    ud_first = fewest_presses(layer - 1, ud + lr + 'A', user_moves)

                user_moves[(layer, ki, kf)] = min(lr_first, ud_first)

    return fewest_presses(layer, code, user_moves)


def part_one(path):
    codes = read_input(path)
    out = 0

    for code in codes:
        out += solve(code, 3) * int(code[:-1])

    return out


def part_two(path):
    codes = read_input(path)
    out = 0

    for code in codes:
        out += solve(code, 26) * int(code[:-1])

    return out


assert part_one('data/21.1.txt') == 126384
print('part one:', part_one('data/21.2.txt'))
print('part two:', part_two('data/21.2.txt'))
