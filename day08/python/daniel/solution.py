
import re
import math
from itertools import cycle

nodes: dict[str, tuple[str, str]] = {}

with open("../../../inputs/input08.txt", "r") as f:
    for i, line in enumerate(f.read().splitlines()):
        if line == "":
            continue
        if i == 0:
            directions = line
        else:
            m = re.findall("[1-9A-Z]+", line)
            nodes[m[0]] = (m[1], m[2])

# Part One
def solve_part_one(directions: str, nodes: dict[str, tuple[str, str]]) -> None:
    current_node = 'AAA'
    for i, d in enumerate(cycle(directions)):
        current_node = nodes[current_node][0 if d == 'L' else 1]
        if current_node == 'ZZZ':
            steps = i+1
            break
    print(steps)

#Part Two
def solve_part_two(directions: str, nodes: dict[str, tuple[str, str]]) -> None:
    start_nodes: list[str] = []
    for node_label in nodes.keys():
        if node_label[-1] == 'A':
            start_nodes.append(node_label)

    steps: list[int] = []

    for node in start_nodes:
        current_node = node
        for i, d in enumerate(cycle(directions)):
            current_node = nodes[current_node][0 if d == 'L' else 1]
            if current_node[-1] == 'Z':
                steps.append(i+1)
                break
            
    print(math.lcm(*steps))
        

solve_part_one(directions, nodes)
solve_part_two(directions, nodes)