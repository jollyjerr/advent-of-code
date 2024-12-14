import re
import math


def read_input(path):
    robots = []
    with open(path, 'r') as file:
        for line in file:
            robots.append(list(map(int, re.findall(r'-?\d+', line))))
    return robots


def part_one(path, width, height, seconds):
    robots = read_input(path)
    mid_row, mid_col = height // 2, width // 2

    quads = [0] * 4
    for (i, [px, py, vx, vy]) in enumerate(robots):
        col = (px + (vx * seconds)) % width
        row = (py + (vy * seconds)) % height
        robots[i][0] = col
        robots[i][1] = row
        if col != mid_col and row != mid_row:
            quadrant = (2 * (row >= mid_row)) + (col >= mid_col)
            quads[quadrant] += 1

    return math.prod(quads)


assert part_one('data/14.1.txt', 11, 7, 100) == 12
print('part one:', part_one('data/14.2.txt', 101, 103, 100))
