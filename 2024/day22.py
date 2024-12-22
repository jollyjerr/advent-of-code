import math


def mix(secret, b):
    return secret ^ b


def prune(secret):
    return secret % 16777216


def calc_next(secret):
    step_one = secret * 64
    secret = prune(mix(secret, step_one))
    step_two = secret // 32
    secret = prune(mix(secret, step_two))
    step_three = secret * 2048
    return prune(mix(secret, step_three))


def offer(secret):
    return int(str(secret)[-1])


class Monkey:
    def __init__(self, initial_secret, iterations=2000):
        self.iterations = iterations
        self.secrets = [initial_secret]
        self.offers = [offer(initial_secret)]
        self.sequences = []
        self.calculate()

    def calculate(self):
        for i in range(1, self.iterations + 1):
            next_val = calc_next(self.secrets[-1])
            self.secrets.append(next_val)
            self.offers.append(offer(next_val))
            if i >= 4:
                self.sequences.append(self.sequence(i))

    def sequence(self, i):
        return (
            self.offers[i - 3] - self.offers[i - 4],
            self.offers[i - 2] - self.offers[i - 3],
            self.offers[i - 1] - self.offers[i - 2],
            self.offers[i] - self.offers[i - 1]
        )

    def score(self, seq):
        if seq not in self.sequences:
            return 0
        idx = self.sequences.index(seq)
        return self.offers[idx + 4]


def read_input(path):
    return [Monkey(int(line.strip())) for line in open(path, "r")]


def part_one(path):
    monkeys = read_input(path)
    return sum(monkey.secrets[-1] for monkey in monkeys)


def part_two(path):
    monkeys = read_input(path)

    all_sequences = set()
    for monkey in monkeys:
        for seq in monkey.sequences:
            all_sequences.add(seq)

    n = len(all_sequences)
    progress = 0

    out = 0
    for i, seq in enumerate(all_sequences):
        le = math.floor((i / n) * 100)
        if le > progress:
            print(f'{le}%')
            progress = le

        score = 0
        for monkey in monkeys:
            score += monkey.score(seq)
        if score > out:
            out = score

    print('out', out)

    return out


# assert part_one('data/22.1.txt') == 37327623
# print('part one:', part_one('data/22.2.txt'))
# assert part_two('data/22.3.txt') == 23
print('part two:', part_two('data/22.2.txt'))
