import re
import math
import os
import time


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
        if col != mid_col and row != mid_row:
            quadrant = (2 * (row >= mid_row)) + (col >= mid_col)
            quads[quadrant] += 1

    return math.prod(quads)


assert part_one('data/14.1.txt', 11, 7, 100) == 12
print('part one:', part_one('data/14.2.txt', 101, 103, 100))


def part_two(path, width, height):
    robots = read_input(path)

    counter = 0

    # bruh is this for real...
    while True:
        if counter > 10000:
            break
        os.system('clear')
        grid = [[0] * width for _ in range(height)]
        for (i, [px, py, vx, vy]) in enumerate(robots):
            grid[py][px] += 1
            col = (px + vx) % width
            row = (py + vy) % height
            robots[i][0] = col
            robots[i][1] = row
        for line in grid:
            print(''.join('*' if x > 0 else ' ' for x in line))
        print(f'does this look like a tree!?!!?? lol ------ {counter}')
        time.sleep(0.5)
        counter += 1


part_two('data/14.2.txt', 101, 103)
