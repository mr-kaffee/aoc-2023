

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


if __name__ == "__main__":
    data = get_data("../../../inputs/input15.txt")
    solve_part_one(data)