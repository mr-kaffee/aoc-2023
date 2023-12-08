import math
import re

def solve(time: list[int], distance: list[int]) -> None:
    product = 1
    for t, d in zip(time, distance):
        D = math.sqrt((t / 2) ** 2 - d)
        d0 = t / 2 - D
        d1 = t / 2 + D
        product *= math.ceil(d1) - math.floor(d0) - 1

    print(product)

with open("../../../inputs/input06.txt", "r") as f:
    input_str = f.read()
    lines = input_str.splitlines()
    time = re.findall("[0-9]+", lines[0])
    distance = re.findall("[0-9]+", lines[1])

solve([int(t) for t in time], [int(d) for d in distance])
solve([int("".join(time))], [int("".join(distance))])