def part_one(input: str) -> int:
    sum: int = 0
    for line in input.splitlines():
        first: int | None = None
        last: int | None = None
        for c in line:
            if c.isdigit():
                if first is None:
                    first = int(c)
                last = int(c)

        if first is not None and last is not None:
            sum += first * 10 + last
    return sum


def part_two(input: str) -> int:
    sum: int = 0
    digits: dict[str, int] = {
        "zero": 0,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    for i in range(10):
        digits[str(i)] = i

    for line in input.splitlines():
        first_index = len(line)
        last_index = 0
        for d, dd in digits.items():
            index = line.find(d)
            if index != -1 and index <= first_index:
                first = dd
                first_index = index

            index = line.rfind(d)
            if index != -1 and index >= last_index:
                last = dd
                last_index = index
        sum += first * 10 + last

    return sum


with open("../../../inputs/input01.txt") as f:
    input = f.read()
    print(part_one(input))
    print(part_two(input))