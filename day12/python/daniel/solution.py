from dataclasses import dataclass
from functools import cache
import re
import time

@dataclass
class Springs:
    pattern: str
    coding: list[int]

def read_input(in_file: str) -> list[Springs]:
    out: list[Springs] = []
    with open(in_file, "r") as f:
        for line in f:
            pattern, coding = line.split()
            out.append(Springs(pattern, [int(i) for i in coding.split(",")]))
    return out

@cache
def find_permutations(pattern: str) -> list[str]:
    if len(pattern) == 1:
        return [".", "#"] if pattern == "?" else [pattern]
    else:
        split = pattern.split("?", 1)
        if len(split) == 1:
            return split
        else:
            result = []
            first, rest = split[0], split[1]
            rest_permutations = find_permutations(rest)
            for p in rest_permutations:
                result.append(first + "." + p)
                result.append(first + "#" + p)
            return result

def solve_part_one(data) -> None:
    result = 0
    for k, s in enumerate(data):
        print(f"Processing line {k+1}", end="")
        valid_permutations = 0
        for perm in find_permutations(s.pattern):
            matches = re.findall("[#]+", perm)
            if len(matches) == len(s.coding) and all( len(m) == c for m, c in zip(matches, s.coding)):
                valid_permutations += 1
        print(f" => {valid_permutations}")
        result += valid_permutations

    print(result)
    return result

def solve_part_two_too_slow(data) -> None:
    result = 0
    for k, s in enumerate(data):
        print(f"Processing line {k+1}", end="")
        valid_permutations = 0
        new_pattern = "?".join([s.pattern]*5)
        new_coding = s.coding*5
        valid_permutations = 0
        for p in find_permutations(new_pattern):
            matches = re.findall("[#]+", ''.join(p))
            if len(matches) == len(new_coding) and all( len(m) == c for m, c in zip(matches, new_coding)):
                valid_permutations += 1
        print(f" => {valid_permutations}")
        result += valid_permutations
    print(result)
    return result


if __name__ == "__main__":
    data = read_input("../../../inputs/input12.txt") 

    start = time.time()
    solve_part_one(data)
    #solve_part_two(data)
    end = time.time()
    print(f"time: {end - start}")

