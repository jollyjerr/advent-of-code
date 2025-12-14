from functools import cache


def read_input(path):
    out = {}
    with open(path, 'r') as file:
        for line in file:
            [key, nodes] = line.strip().split(':')
            out[key] = nodes.split(' ')[1:]
    return out


def part_one(path):
    graph = read_input(path)
    visited = set()

    def dfs(node):
        if node == 'out':
            return 1

        total = 0
        for n in graph[node]:
            if n not in visited:
                visited.add(n)
                total += dfs(n)
                visited.remove(n)
        return total

    return dfs('you')


assert part_one('data/11.1.txt') == 5
print('pt one', part_one('data/11.2.txt'))


def part_two(path):
    graph = read_input(path)
    visited = set()

    @cache
    def dfs(node, has_fft, has_dac):
        if node == 'out':
            out = 1 if has_fft and has_dac else 0
            return out

        total = 0
        for n in graph[node]:
            if n not in visited:
                fft = has_fft
                dac = has_dac
                if n == 'fft':
                    fft = True
                elif n == 'dac':
                    dac = True
                visited.add(n)
                total += dfs(n, fft, dac)
                visited.remove(n)
        return total

    return dfs('svr', False, False)


assert part_two('data/11.3.txt') == 2
print('pt two', part_two('data/11.2.txt'))
