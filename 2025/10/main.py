import re
import pulp


class Machine:
    def __init__(self, line):
        self.lights = self.extract_lights(line)
        self.buttons = self.extract_buttons(line)
        self.joltage = self.extract_joltage(line)

    def extract_lights(self, line):
        match = re.search(r'\[(.*?)\]', line)
        return match.group(1) if match else None

    def extract_buttons(self, line):
        matches = re.findall(r'\((.*?)\)', line)
        return [list(map(int, match.split(','))) for match in matches]

    def extract_joltage(self, line):
        match = re.search(r'\{(.*?)\}', line)
        return list(map(int, match.group(1).split(','))) if match else None

    def change(self, start, button):
        out = ""
        for i in range(len(start)):
            if i in button:
                ch = '#' if start[i] == '.' else '.'
                out += ch
            else:
                out += start[i]
        return out

    def min_press_configure_lights(self):
        return self.shortest_path("." * len(self.lights), self.lights)

    def shortest_path(self, start, end):
        queue = [(start, 0)]
        visited = {start}

        while queue:
            current, distance = queue.pop(0)

            if current == end:
                return distance

            for button in self.buttons:
                new_state = self.change(current, button)

                if new_state not in visited:
                    visited.add(new_state)
                    queue.append((new_state, distance + 1))

    def min_press_configure_joltage(self):
        m = len(self.joltage)
        k = len(self.buttons)

        prob = pulp.LpProblem("MinPresses", pulp.LpMinimize)
        x = [pulp.LpVariable(f"x{j}", lowBound=0, cat='Integer') for j in range(k)]

        prob += pulp.lpSum(x)

        for p in range(m):
            prob += pulp.lpSum(x[j] for j in range(k) if p in self.buttons[j]) == self.joltage[p]

        status = prob.solve(pulp.PULP_CBC_CMD(msg=0))

        if status != 1:
            print("unsolvable")
            return None

        return int(pulp.value(prob.objective))


def read_input(path):
    return [Machine(line.strip()) for line in open(path, 'r')]


def part_one(path):
    return sum(machine.min_press_configure_lights() for machine in read_input(path))


assert part_one('data/10.1.txt') == 7
print('pt one:', part_one('data/10.2.txt'))


def part_two(path):
    return sum(machine.min_press_configure_joltage() for machine in read_input(path))


assert part_two('data/10.1.txt') == 33
print('pt two:', part_two('data/10.2.txt'))
