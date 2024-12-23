from collections import defaultdict


def read_input(path):
    connections = defaultdict(list)
    with open(path, "r") as file:
        for line in file:
            [host, client] = line.strip().split('-')
            connections[host].append(client)
            connections[client].append(host)
    return connections


def part_one(path):
    graph = read_input(path)

    groups = set()
    for node in graph:
        n = graph[node]
        for i in range(len(n)):
            for j in range(i + 1, len(n)):
                if n[j] in graph[n[i]]:
                    if node.startswith('t') or n[i].startswith('t') or n[j].startswith('t'):
                        groups.add(tuple(sorted([node, n[i], n[j]])))

    return len(groups)


assert part_one('data/23.1.txt') == 7
print('part one:', part_one('data/23.2.txt'))


def part_two(path):
    graph = read_input(path)

    def bron_kerbosch(r, p, x, cliques):
        if p == set() and x == set():
            cliques.append(r.copy())
            return
        p_copy = p.copy()
        for v in p_copy:
            neighbors = graph[v]
            bron_kerbosch(
                r.union({v}),
                p.intersection(neighbors),
                x.intersection(neighbors),
                cliques
            )
            p.remove(v)
            x.add(v)

    cliques = []
    r = set()
    p = set(graph.keys())
    x = set()

    bron_kerbosch(r, p, x, cliques)

    return ','.join(sorted(list(sorted(cliques, key=len)[-1])))


assert part_two('data/23.1.txt') == 'co,de,ka,ta'
print('part two:', part_two('data/23.2.txt'))
