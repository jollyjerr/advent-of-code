from itertools import product


def read_input(path):
    lines = []
    with open(path, 'r') as file:
        for line in file:
            [target, vals] = line.strip().split(': ')
            lines.append((int(target), [int(x) for x in vals.split(' ')]))
    return lines


def part_one(path):
    equations = read_input(path)
    found = []

    for (target, numbers) in equations:
        operator_sets = product(["+", "*"], repeat=(len(numbers) - 1))

        for operators in operator_sets:
            result = numbers[0]
            for (number, operator) in zip(numbers[1:], operators):
                if operator == "+":
                    result += number
                else:
                    result *= number
            if result == target:
                found.append(target)
                break

    return sum(found)


assert part_one('data/7.1.txt') == 3749
print('part one:', part_one('data/7.2.txt'))


def part_two(path):
    equations = read_input(path)
    found = []

    for (target, numbers) in equations:
        operator_sets = product(["+", "*", "||"], repeat=(len(numbers) - 1))

        for operators in operator_sets:
            result = numbers[0]
            for (number, operator) in zip(numbers[1:], operators):
                if result > target:
                    break

                if operator == "+":
                    result += number
                elif operator == "*":
                    result *= number
                elif operator == "||":
                    result = int(str(result) + str(number))

            if result == target:
                found.append(target)
                break

    return sum(found)


assert part_two('data/7.1.txt') == 11387
print('part two:', part_two('data/7.2.txt'))
