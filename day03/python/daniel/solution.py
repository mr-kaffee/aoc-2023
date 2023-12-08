from collections import defaultdict
from pathlib import Path


def make_grid(input_file: Path) -> list[list[str]]:
    grid: list[list[str]] = []
    with open(input_file) as f:
        for line in f:
            grid.append([c for c in line if c != '\n'])
    return grid


def check_if_part_of_gear(i: int, j: int, grid: list[list[str]]) -> tuple[int] | None:
    neighbours: list[tuple[int]] = [(-1, 0), (+1, 0), (0, -1), (0, +1), (-1, -1), (-1, +1), (+1, -1), (+1, +1)]

    for ii, jj in neighbours:
        try:
            if grid[i+ii][j+jj] == "*":
                return (i+ii, j+jj) 
        except IndexError:
            continue

    return None


def solve_part_2(grid: list[list[str]]) -> int:
    gears: defaultdict[tuple[int], list[int]] = defaultdict(list)

    for i, row in enumerate(grid):
        recording_number: bool = False
        gear_index: tuple[int] | None = None

        for j, c in enumerate(row):
            if not recording_number:
                if c.isdecimal():
                    number = int(c) 
                    recording_number = True
            else:
                if c.isdecimal():
                    number = number * 10 + int(c)
                else:
                    if gear_index is not None:
                        gears[gear_index].append(number)

                    recording_number = False
                    gear_index = None

            if recording_number and gear_index is None:
                gear_index = check_if_part_of_gear(i, j, grid)

        if recording_number and gear_index is not None:
            gears[gear_index].append(number)
        
    return sum(gear[0]*gear[1] for gear in gears.values() if len(gear) == 2)

grid = make_grid(Path("../../../inputs/input03.txt"))
solution = solve_part_2(grid)
print(f"Solution Part 2: {solution}")
