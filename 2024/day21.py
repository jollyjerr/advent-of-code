from functools import cache


numpad = {
    # (0, 0): '#',
    (0, 1): '0',
    (0, 2): 'A',  # initial position
    (1, 0): '1',
    (1, 1): '2',
    (1, 2): '3',
    (2, 0): '4',
    (2, 1): '5',
    (2, 2): '6',
    (3, 0): '7',
    (3, 1): '8',
    (3, 2): '9',
}


dirpad = {
    # (0, 0): '#',
    (0, 1): '^',
    (0, 2): 'A',  # initial position
    (-1, 0): '<',
    (-1, 1): 'v',
    (-1, 2): '>'
}


graphs = {
    'numpad': numpad,
    'dirpad': dirpad
}


DIRPAD = 'dirpad'
NUMPAD = 'numpad'


# <, ^, >, v
directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]


@cache
def rev_graph(graphname):
    return {value: key for key, value in graphs[graphname].items()}


@cache
def neighbors(graphname, node):
    graph = graphs[graphname]
    (y, x) = node
    options = [(y + dy, x + dx) for (dy, dx) in directions]
    return [o for o in options if o in graph]


@cache
def all_paths(graphname, start, end):
    out = []

    def dfs(current, path):
        path.append(current)
        if current == end:
            out.append(path[:])
        else:
            for n in neighbors(graphname, current):
                if n not in path:
                    dfs(n, path)
        path.pop()

    dfs(start, [])
    return sorted(out, key=len)


@cache
def num_code_to_dir_codes(code):
    rev = rev_graph(NUMPAD)
    position = (0, 2)

    options = set()

    for char in list(code):
        paths = all_paths(NUMPAD, position, rev[char])
        if options:
            for option in options:
                options.remove(option)
                for path in paths:
                    options.add(option + path)
        else:
            for path in paths:
                options.add(path)

    return options


@cache
def shortest_path(code, graphname, start):
    hparg = rev_graph(graphname)

    position = start
    full_path = []

    for char in list(code):
        full_path += all_paths(graphname, position, hparg[char])[0]
        position = hparg[char]

    return full_path


def read_input(path):
    return [line.strip() for line in open(path, "r")]


def part_one(path):
    codes = read_input(path)

    # robots = [(0, 2), (0, 2), (0, 2)]

    example = codes[0]
    print(example)
    print(num_code_to_dir_codes(example))

    return 0


assert part_one('data/21.1.txt') == 126384
