import os


def read_input(path):
    map = []
    start = ()
    instructions = []

    with open(path, 'r') as file:
        for i, line in enumerate(file):
            if '#' in line:
                row = list(line.strip())
                for j, item in enumerate(row):
                    if item == '@':
                        start = (i, j)
                map.append(row)
            else:
                instructions += list(line.strip())

    return (map, start, instructions)


commands = {
    '<': (0, -1),
    '^': (-1, 0),
    '>': (0, 1),
    'v': (1, 0)
}


def part_one(path):
    (map, start, instructions) = read_input(path)

    robot_location = start
    for command in instructions:
        delta = commands[command]
        current_item = robot_location
        search_pointer = (current_item[0] + delta[0], current_item[1] + delta[1])
        can_push = False
        stack = []
        while map[search_pointer[0]][search_pointer[1]] != '#':
            stack.insert(0, current_item)
            if map[search_pointer[0]][search_pointer[1]] == '.':
                can_push = True
                break
            current_item = search_pointer
            search_pointer = (current_item[0] + delta[0], current_item[1] + delta[1])
        if can_push:
            if len(stack) > 0:
                robot_location = (robot_location[0] + delta[0], robot_location[1] + delta[1])
            for item in stack:
                new_spot = (item[0] + delta[0], item[1] + delta[1])
                map[new_spot[0]][new_spot[1]] = map[item[0]][item[1]]
                map[item[0]][item[1]] = '.'

    cords = []
    for i in range(len(map)):
        for j in range(len(map[0])):
            if map[i][j] == 'O':
                cords.append(100 * i + j)

    return sum(cords)


assert part_one('data/15.1.txt') == 2028
assert part_one('data/15.2.txt') == 10092
print('part one:', part_one('data/15.3.txt'))


def expand_map(map):
    expanded_map = []
    start = ()
    for i, row in enumerate(map):
        expanded_row = []
        for j, item in enumerate(row):
            if item == '#' or item == '.':
                expanded_row.append(item)
                expanded_row.append(item)
            elif item == 'O':
                expanded_row.append('[')
                expanded_row.append(']')
            elif item == '@':
                start = (i, len(expanded_row))
                expanded_row.append(item)
                expanded_row.append('.')
        expanded_map.append(expanded_row)
    return (expanded_map, start)


def discover_full_box(map, i, j, seen, delta):
    if (i, j) in seen:
        return
    item = map[i][j]

    if item == '[':
        seen.add((i, j))
        seen.add((i, j + 1))
    elif item == ']':
        seen.add((i, j))
        seen.add((i, j - 1))
    else:
        return

    copy = [x for x in seen]
    for (x, y) in copy:
        discover_full_box(map, x + delta[0], y + delta[1], seen, delta)


DEBUG = False


def part_two(path):
    (map, start, instructions) = read_input(path)
    (map, start) = expand_map(map)

    robot_location = start
    for command in instructions:
        delta = commands[command]
        next_robot_location = (robot_location[0] + delta[0], robot_location[1] + delta[1])
        next_block = map[next_robot_location[0]][next_robot_location[1]]
        if next_block == '.':
            map[next_robot_location[0]][next_robot_location[1]] = '@'
            map[robot_location[0]][robot_location[1]] = '.'
            robot_location = next_robot_location
        elif next_block == '[' or next_block == ']':
            seen = set()
            discover_full_box(map, next_robot_location[0], next_robot_location[1], seen, delta)
            can_move = all(map[x + delta[0]][y + delta[1]] != '#' for (x, y) in seen)
            if can_move:
                goals = [(map[i][j], i + delta[0], j + delta[1]) for (i, j) in seen]
                destinations = [(x, y) for (_, x, y) in goals]
                for (item, x, y) in goals:
                    map[x][y] = item
                for (i, j) in seen:
                    if (i, j) not in destinations:
                        map[i][j] = '.'
                map[next_robot_location[0]][next_robot_location[1]] = '@'
                map[robot_location[0]][robot_location[1]] = '.'
                robot_location = next_robot_location

        if DEBUG:
            os.system('clear')
            print('----------------', command, '-------------')
            if len(seen) > 0:
                print(seen)
            for row in map:
                print(''.join(row))
            input('enter to continue')

    if DEBUG:
        print('---------- done --------------')
        for row in map:
            print(''.join(row))

    cords = []
    for i in range(len(map)):
        for j in range(len(map[0])):
            if map[i][j] == '[':
                cords.append(100 * i + j)

    return sum(cords)


assert part_two('data/15.2.txt') == 9021
print('part two:', part_two('data/15.3.txt'))
