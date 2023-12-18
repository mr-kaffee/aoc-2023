from dataclasses import dataclass
from collections import deque

@dataclass(frozen=True)
class Vec:
    x: int
    y: int

    def __add__(self, other):
        return Vec(self.x + other.x, self.y + other.y)


@dataclass
class Beam:
    heads: deque[tuple[Vec, Vec]]
    visited: set[tuple[Vec, Vec]]


def get_data(input_file: str) -> list[str]:
    data: list[str] = []
    with open(input_file, "r") as f:
        for line in f:
                data.append(line.strip())
    return data


def in_bounds(data: list[str], pos: Vec) -> bool:
    return pos.x >= 0 and pos.x < len(data) and pos.y >= 0 and pos.y < len(data[0])


def get_new_directions(data: list[str], pos: Vec, dir: Vec) -> list[Vec]:
    result: list[Vec] = []
    
    match data[pos.x][pos.y]:
        case "/": 
            result.append(Vec(-1*dir.y, -1*dir.x))
        case "\\":
            result.append(Vec(dir.y, dir.x))
        case "-" if dir.x != 0:
            result.append(Vec(0, +1))
            result.append(Vec(0, -1))
        case "|" if dir.y != 0:
            result.append(Vec(+1, 0))
            result.append(Vec(-1, 0))
        case _:
            result.append(dir)
    
    return result

def update_head_position(data: list[str], pos: Vec, dir: Vec, visited: set[tuple[Vec, Vec]]) -> Vec | None:
    new_pos = pos + dir
    if not in_bounds(data, new_pos) or (new_pos, dir) in visited:
        return None
    else:
        return new_pos



def solve_part_one(data: list[str]) -> None:
    b = Beam(deque(), set())
    b.heads.append( (Vec(0,0), Vec(0,1)) )

    while b.heads:
        h = b.heads.popleft()
        b.visited.add(h)

        dirs = get_new_directions(data, h[0], h[1])
        for d in dirs:
            if (new_pos := update_head_position(data, h[0], d, b.visited)) is not None:
                b.heads.append((new_pos, d))

    result = set()
    for v in b.visited:
        result.add(v[0])
    print(f"Solution part one: {len(result)}")


if __name__ == "__main__":
    data = get_data("../../../inputs/input16.txt")
    solve_part_one(data)