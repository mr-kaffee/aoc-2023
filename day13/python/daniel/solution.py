import math

def get_data(input_file: str) -> list[list[str]]:
    data: list[list[str]] = [[]]
    with open(input_file, "r") as f:
        for line in f:
            if line == "\n":
                data.append([])
            else:
                data[-1].append(line.strip())
    return data

def transpose(data: list[str]) -> list[str]:
    transposed_data: list[str] = []
    for j in range(len(data[0])):
        row: list[str] = []
        for i in range(len(data)):
            row.append(data[i][j])
        transposed_data.append("".join(row))
    return transposed_data

def find_potential_starting_points(data: list[str]) -> list[float]:
    result: list[float] = []
    for i in range(len(data)-1):
        if data[i] == data[i+1]:
            result.append(i+0.5)
    return result

def get_mirror_points(data: list[list[str]]) -> list[int]:
    number_of_rows = len(data) 
    mirror_points: list[int] = []
    for s in find_potential_starting_points(data):
        r = min(math.ceil(s), number_of_rows - math.ceil(s))
        mirror = True
        for i in range(r):
            if data[int(s-(2*i+1)*0.5)] != data[int(s+(2*i+1)*0.5)]:
                mirror = False
                break
        if mirror:
            mirror_points.append(s)
    return mirror_points


def solve_part_one(data: list[list[str]]) -> None:
    result = 0
    for pattern in data:
        mc = get_mirror_points(transpose(pattern))
        for m in mc:
            result += math.ceil(m)

        mr = get_mirror_points(pattern)
        for m in mr:
            result += 100*math.ceil(m)

        print(mc, mr)
    print(result)

if __name__ == "__main__":
    data = get_data("../../../inputs/input13.txt")
    solve_part_one(data)