from collections import defaultdict
from pathlib import Path


def parse_input(input_file: Path):
    winning = []
    your = []
    with open(input_file) as f:
        for i, line in enumerate(f):
            winning.append(line.removeprefix(f"Card {i+1:3d}:").split("|")[0].split())
            your.append(line.removeprefix(f"Card {i+1:3d}:").split("|")[1].split())
    return winning, your
                

def solve_part_one(winning: list[tuple[list[int]]], you) -> None:
    sum = 0
    for w, y in zip(winning, you):
        score = None
        for wn in w:
            if wn in y and score is None:
                score = 1
            elif wn in y:
                score *= 2
        if score is not None:
            sum += score
    print(f"Solution Part 1: {sum}")

def solve_part_two(winning, you) -> None:
    total_number_of_cards = [1 for _ in range(len(winning))]
    for cn, (w, y) in enumerate(zip(winning, you)):
        wins = 0
        for wn in w:
            if wn in y:
                wins += 1
        for i in range(wins):
            try:
                total_number_of_cards[cn+i+1] += total_number_of_cards[cn]
            except IndexError:
                break

    total = sum(total_number_of_cards)
    print(f"Solution Part 2: {total}")

winning, you = parse_input(Path("../../../inputs/input04.txt"))
solve_part_one(winning, you)
solve_part_two(winning, you)
