from collections import OrderedDict

Universe = list[list[str]]

def read_input(in_file: str) -> Universe:
    out: Universe = []

    with open(in_file, "r") as f:
        for line in f.read().splitlines():
            out.append(line)
    return out

def transpose(universe: Universe) -> Universe:
    transposed_universe: Universe = [[0]*len(universe) for _ in range(len(universe[0]))]
    for i, line in enumerate(universe):
        for j, symbol in enumerate(line):
            transposed_universe[j][i] = symbol
    return transposed_universe
    
def find_galaxies(universe: Universe) -> OrderedDict[int, tuple[int, int]]:
    out: OrderedDict[int, tuple[int, int]] = OrderedDict()
    count = 0
    for i, line in enumerate(universe):
        for j, symbol in enumerate(line):
            if symbol == '#':
                out[count] = (i, j)
                count += 1

    return out

def find_empty_rows(universe: Universe) -> list[int]:
    out: list[int] = []
    for i, line in enumerate(universe):
        if '#' not in line:
            out.append(i)
    return out

def find_empty_columns(universe: Universe) -> list[int]:
    out: list[int] = []
    transposed_universe = transpose(universe)
    for j, line in enumerate(transposed_universe):
        if '#' not in line:
            out.append(j)
    return out

def find_empty_between(start: int, end: int, empty_indices: list[int]) -> int:
    out = 0
    for ei in empty_indices:
        if start < ei < end or end < ei < start:
            out += 1
    return out

def sum_distances(universe: Universe, factor: int) -> int:
    galaxies = find_galaxies(universe)
    empty_rows = find_empty_rows(universe)
    empty_columns = find_empty_columns(universe)
    sum = 0
    for gn, gi in galaxies.items():
        for gnn, gii in galaxies.items():
            if gnn <= gn:
                continue
            else:
                dist_x = abs(gi[0]-gii[0]) + find_empty_between(gi[0], gii[0], empty_rows) * (factor - 1)
                dist_y = abs(gi[1]-gii[1]) + find_empty_between(gi[1], gii[1], empty_columns) * (factor - 1)

                sum += dist_x + dist_y
    return sum

def solve_part_one(universe: Universe) -> None:
    print(f"Solution part one: {sum_distances(universe, factor=2)}")

def solve_part_two(universe: Universe) -> None:
    print(f"Solution part two: {sum_distances(universe, factor=1_000_000)}")


if __name__ == "__main__":
    universe = read_input("../../../inputs/input11.txt")
    solve_part_one(universe)
    solve_part_two(universe)


