class CPU:
    def __init__(self, a, b, c):
        self.A = a
        self.B = b
        self.C = c
        self.instruction = 0

    def run(self, program):
        out = []
        while self.instruction < len(program):
            opcode = program[self.instruction]
            operand = program[self.instruction + 1]
            combo = self.decode_combo(operand)

            if opcode == 0:
                self.A //= (2 ** combo)
            elif opcode == 1:
                self.B ^= operand
            elif opcode == 2:
                self.B = combo % 8
            elif opcode == 3:
                if self.A != 0:
                    self.instruction = operand
                    continue
            elif opcode == 4:
                self.B ^= self.C
            elif opcode == 5:
                out.append(str(combo % 8))
            elif opcode == 6:
                self.B = self.A // (2 ** combo)
            elif opcode == 7:
                self.C = self.A // (2 ** combo)
            else:
                raise ValueError()

            self.instruction += 2
        return ','.join(out)

    def decode_combo(self, operand):
        if 0 <= operand <= 3:
            return operand
        elif operand == 4:
            return self.A
        elif operand == 5:
            return self.B
        elif operand == 6:
            return self.C
        else:
            raise ValueError()


def read_input(path):
    with open(path, 'r') as file:
        a = int(file.readline().split(":")[1].strip())
        b = int(file.readline().split(":")[1].strip())
        c = int(file.readline().split(":")[1].strip())
        file.readline()
        program = list(map(int, file.readline().split(" ")[1].split(",")))
        return a, b, c, program


def part_one(path):
    a, b, c, program = read_input(path)
    cpu = CPU(a, b, c)
    return cpu.run(program)


assert part_one('data/17.1.txt') == '4,6,3,5,6,3,5,2,1,0'
print('part one:', part_one('data/17.2.txt'))


def part_two(path):
    _a, b, c, program = read_input(path)
    guess = 0
    while True:
        result = [int(x) for x in CPU(guess, b, c).run(program).split(',')]
        result.reverse()

        all = True
        for i, val in enumerate(result):
            if val != program[len(program) - (i + 1)]:
                all = False
                break

        if all and (len(result) == len(program)):
            return guess

        if guess == 0 or not all:
            guess += 1
        else:
            guess *= 8


print('part two:', part_two('data/17.2.txt'))

# searching forward number by number:
# 16 -> 485 -> 6653 ->
# idk...

# searching backward number by number:
# 0 -> 24 -> 196 -> 1573
# multiply last match by 8 (ish) ?
