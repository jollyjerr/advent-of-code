import re


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

    def change_lights(self, start, button):
        out = ""
        for i in range(len(start)):
            if i in button:
                ch = '#' if start[i] == '.' else '.'
                out += ch
            else:
                out += start[i]
        return out

    def change_joltage(self, start, button):
        out = []
        for i in range(len(start)):
            ch = start[i] + 1 if i in button else start[i]
            out.append(ch)
        return out

    def min_press_configure_lights(self):
        return self.shortest_path_lights("." * len(self.lights), self.lights)

    def min_press_configure_joltage(self):
        return self.shortest_path_joltage([0] * len(self.joltage), self.joltage)

    def shortest_path_lights(self, start, end):
        queue = [(start, 0)]
        visited = {start}

        while queue:
            current, distance = queue.pop(0)
            print(current, distance)

            if current == end:
                return distance

            for button in self.buttons:
                new_state = self.change_lights(current, button)

                if new_state not in visited:
                    visited.add(new_state)
                    queue.append((new_state, distance + 1))

    def shortest_path_joltage(self, start, end):
        start_key = tuple(start)

        queue = [(start_key, 0)]
        visited = {start_key}

        while queue:
            current, distance = queue.pop(0)

            if current == end:
                print('done')
                return distance

            for button in self.buttons:
                new_state = self.change_joltage(current, button)
                new_key = tuple(new_state)
                if new_key not in visited:
                    visited.add(new_key)
                    queue.append((new_state, distance + 1))


def read_input(path):
    return [Machine(line.strip()) for line in open(path, 'r')]


def part_one(path):
    return sum(machine.min_press_configure_lights() for machine in read_input(path))


# assert part_one('data/10.1.txt') == 7
# print('pt one:', part_one('data/10.2.txt'))


def part_two(path):
    return sum(machine.min_press_configure_joltage() for machine in read_input(path))


assert part_two('data/10.1.txt') == 33
print('pt two:', part_two('data/10.2.txt'))
