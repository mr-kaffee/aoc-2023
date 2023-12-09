sequences: list[int] = []
with open("../../../inputs/input09.txt") as f:
    for line in f:
        sequences.append([int(v) for v in line.split()])

def solve_part_one(sequences: list[int]) -> None:
    new_values: list[int] = []
    for seq in sequences:
        diff_sequences: list[list[int]] = [seq]
        while sum(diff_sequences[-1]) != 0:
            diff_sequences.append([])
            for i, j in zip(diff_sequences[-2], diff_sequences[-2][1:]):
                diff_sequences[-1].append(j-i)
        
        new_values.append(sum(dseq[-1] for dseq in diff_sequences))

    print(f"Solution part one: {sum(new_values)}")

def solve_part_two(sequences: list[int]) -> None:
    new_values: list[int] = []
    for seq in sequences:
        diff_sequences: list[list[int]] = [seq]
        while sum(diff_sequences[-1]) != 0:
            diff_sequences.append([])
            for i, j in zip(diff_sequences[-2], diff_sequences[-2][1:]):
                diff_sequences[-1].append(j-i)

        for i in range(len(diff_sequences)-2, -1, -1):
            diff_sequences[i].insert(0, diff_sequences[i][0] -  diff_sequences[i+1][0])
        new_values.append(diff_sequences[0][0])

    print(f"Solution part two: {sum(new_values)}")

solve_part_one(sequences)
solve_part_two(sequences)