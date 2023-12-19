from dataclasses import dataclass, field


@dataclass(eq=False)
class Lense:
    label: str
    focal_length: int

    def __eq__(self, other) -> bool:
        return self.label == other.label

@dataclass
class Box:
    lenses: list[Lense] = field(default_factory=lambda: [])


def get_data(input_file: str) -> str:
    with open(input_file, "r") as f:
        return f.read().strip()


def calculate_hash(string: str) -> int:
    result = 0
    for s in string:
        result += ord(s)
        result *= 17
        result %= 256
    return result


def solve_part_one(data: str) -> None:
    result = 0

    for sub_string in data.split(","):
        result += calculate_hash(sub_string)

    print(f"Solution part one: {result}")


def solve_part_two(data: str) -> None:
    boxes: list[Box] = []
    for _ in range(256):
        boxes.append(Box())

    for sub_string in data.split(","):
        if '=' in sub_string:
            label, focal_length = sub_string.split("=")
            box_number = calculate_hash(label) 
            try:
                index = boxes[box_number].lenses.index(Lense(label, 0))
                boxes[box_number].lenses[index].focal_length = int(focal_length)
            except ValueError:
                boxes[box_number].lenses.append(Lense(label, int(focal_length)))
        elif '-' in sub_string:
            label, *_ = sub_string.split("-")
            box_number = calculate_hash(label)

            try:
                boxes[box_number].lenses.remove(Lense(label, 1))
            except ValueError:
                pass

    result = 0
    for i, b in enumerate(boxes):
        for j, l in enumerate(b.lenses):
            result += (i+1)*(j+1)*l.focal_length

    print(f"Solution part two: {result}")


if __name__ == "__main__":
    data = get_data("../../../inputs/input15.txt")
    solve_part_one(data)
    solve_part_two(data)