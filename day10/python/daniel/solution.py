from typing import Callable

DIRECTIONS: dict[str, list[tuple[int, int]]] = {
    "|": [(+1, 0), (-1, 0)],
    "-": [(0, +1), (0, -1)],
    "L": [(-1, 0), (0, +1)],
    "J": [(-1, 0), (0, -1)],
    "7": [(+1, 0), (0, -1)],
    "F": [(+1, 0), (0, +1)],
    ".": [],
}

MAPPING: dict[str, str] = {
    "|": "\u2503",
    "-": "\u2501",
    "L": "\u2517",
    "J": "\u251B",
    "7": "\u2513",
    "F": "\u250F",
    ".": ".",
    "S": "S",
}

def pretty_print(grid: list[list[str]], inner: list[tuple[int, int]], loop=list[tuple[int, int]]) -> None:
    for i, line in enumerate(grid):
        for j, char in enumerate(line):
            if inner is not None and (i,j) in inner:
                print("I", sep="", end="")
            elif loop is not None and (i, j) in loop:
                print(MAPPING[char], sep="", end="")
            else:
                print("O", sep="", end="")
        print()


def find_start(grid: list[list[str]]) -> tuple[int, int]:
    for i, line in enumerate(grid):
        for j, char in enumerate(line):
            if char == "S":
                return i, j


def get_start_directions(grid: list[list[str]], start: tuple[int, int]) -> list[tuple[int, int]]:
    result: list[tuple[int, int]] = [] 
    si, sj = start
    for (i,j), c in zip([(+1, 0), (-1, 0), (0, +1), (0, -1)], [("|", "J", "L"), ("|", "7", "F"), ("-", "J", "7"), ("-", "F", "L")]):
        try:
           if grid[si + i][sj + j]  in c:
               result.append((i,j))
        except IndexError:
            continue
    return result


def get_loop(grid: list[list[str]]) -> list[tuple[int, int]]:
    si, sj = find_start(grid)
    di, dj = get_start_directions(grid, (si, sj))[0]
    current_index = si + di, sj + dj
    previous = (si, sj)
    loop_tiles = [current_index]

    while True: 
        i, j = current_index
        for d in DIRECTIONS[grid[i][j]]:
            new = i + d[0], j + d[1]
            if new != previous:
                loop_tiles.append(new)
                break
        previous, current_index = current_index, new

        if current_index == (si, sj):
            break
    return loop_tiles


def get_connected_area(grid: list[list[str]], 
                       loop: list[str],
                       start: tuple[int, int], 
                       expand: Callable[[list[list[str]], list[str], tuple[int, int]], list[tuple[int, int]]]) -> list[tuple[int, int]]:
    fringe: list[int, int] = [start]
    explored: list[int, int] = []

    while len(fringe) != 0:
        current = fringe.pop()
        if current in explored:
            continue
        expanded = expand(grid, loop, current)
        explored.append(current)
        for index_tuple in expanded:
            if index_tuple not in fringe:
                fringe.append(index_tuple)
    return explored


def expand(grid, loop, index) -> list[tuple[int, int]]:
    out = []
    i, j = index
    for ii,jj in [(+1, 0), (-1, 0), (0, +1), (0, -1)]:
        ni, nj = i+ii, j+jj
        if ni < 0 or nj < 0 or ni >= len(grid) or nj >= len(grid[0]):
            continue
        if (ni, nj) in loop:
            continue

        out.append((ni, nj))
    return out

def is_inside(index: tuple[int, int], loop: list[tuple[int, int]]) -> bool:
    tiles=[]
    i, j = index
    for ii, jj in  [(+1, 0), (0, +1), (-1, 0), (0, -1)]:
        n = 1
        while True:
            next_index = i+n*ii, j+n*jj

            # out of bound check
            try:
                grid[next_index[0]][next_index[1]]
            except IndexError:
                break

            if next_index in loop:
                tiles.append(next_index)
                break
            n+= 1
    
    if len(tiles) != 4:
        return False

    order = []
    for index in loop:
        if index in tiles: 
            order.append(index)
        if len(order) == 4:
            break

    for i, index in enumerate(order):
        if tiles[0] == index:
            start_i = i
    
    return tiles[1] == order[(start_i + 1) % len(order)]

def solve_part_one(grid: list[list[str]]) -> None:
    start_index = find_start(grid)
    DIRECTIONS["S"] = get_start_directions(grid, start_index)
    previous = [] 
    current_indices = [find_start(grid)]
    steps = 0
    while True:
        new_indices = []
        for i,j in current_indices:
            for d in DIRECTIONS[grid[i][j]]:
                new = i + d[0], j + d[1]
                if new not in previous:
                    new_indices.append(new)
        previous = current_indices
        current_indices = new_indices 
        steps += 1
        if current_indices[0] == current_indices[1]:
            break

    print(f"Solution part one: {steps}")

def solve_part_two(grid:list[list[str]]) -> None:
    loop = get_loop(grid)
    connected_areas: list[list[tuple[int, int]]] = []
    for i, line in enumerate(grid):
        for j, _ in enumerate(line):
            if (i, j) in loop:
                continue
            skip = False
            for ca in connected_areas:
                if (i,j) in ca:
                    skip = True
                    break
            if not skip:
                connected_areas.append(get_connected_area(grid, loop, (i,j), expand))

    inner = []
    for ca in connected_areas:
        if is_inside(ca[0], loop):
            for i in ca:
                inner.append(i)

    pretty_print(grid, inner, loop)
    print(f"Solution part two: {len(inner)}")

if __name__ == "__main__":
    grid: list[list[str]] = []
    with open("../../../inputs/input10.txt", "r") as f:
        for line in f.read().splitlines():
            grid.append(line)

    solve_part_one(grid=grid)
    solve_part_two(grid=grid)