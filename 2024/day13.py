def read_input(path):
    games = []
    with open(path, 'r') as file:
        game = []
        # edited my input files with vim to avoid parser
        for line in file:
            if line.strip() == '':
                games.append(game)
                game = []
                continue
            vals = [int(x) for x in line.strip().split(' ')]
            game.append((vals[0], vals[1]))
    return games


def part_one(path, offset=0):
    games = read_input(path)

    total_cost = 0
    for game in games:
        [a, b, prize] = game

        prize = (prize[0] + offset, prize[1] + offset)

        # cramer's rule for solving systems of equations. Had to google this :(
        m_determinant = (a[0] * b[1] - a[1] * b[0])
        presses_a = (prize[0] * b[1] - prize[1] * b[0]) / m_determinant
        presses_b = (prize[1] * a[0] - prize[0] * a[1]) / m_determinant

        if presses_a == int(presses_a) and presses_b == int(presses_b):
            total_cost += (presses_a * 3) + (presses_b * 1)

    return int(total_cost)


assert part_one('data/13.1.txt') == 480
print('part one:', part_one('data/13.2.txt'))

# 91036
# 30393
# 29839

print('part two:', part_one('data/13.2.txt', 10000000000000))
