from dataclasses import dataclass

@dataclass
class Range:
    start: int
    length: int

def solve_part_one(seeds: list[int], in_out_mappings: list[list[int]]) -> None:
    locations = []
    for s in seeds:
        current_mapping = s
        for maps in in_out_mappings:
            new_mapping = None
            for range in maps:
                if current_mapping < range[1] or current_mapping > range[1] + range[2]:
                    continue
                else:
                    new_mapping = range[0] + current_mapping - range[1] 
            if new_mapping is not None:
                current_mapping = new_mapping

        locations.append(current_mapping)

    print(min(locations))

def get_mapped_ranges(in_ranges: list[Range], in_out_mapping: list[list[int]]) -> list[Range]:
    out_ranges: list[Range] = []
    ranges_to_process = in_ranges
    for mapping in in_out_mapping:
        open_ranges: list[Range] = []
        for in_range in ranges_to_process:
            in_range_end = in_range.start + in_range.length - 1
            mapping_end = mapping[1] + mapping[2] - 1

            if in_range_end < mapping[1] or in_range.start > mapping_end:
                open_ranges.append(in_range)
                continue

            if in_range.start >= mapping[1] and in_range_end <= mapping_end:
                offset = in_range.start - mapping[1]
                out_ranges.append(Range(mapping[0] + offset, in_range.length))

            elif in_range.start < mapping[1] and in_range_end <= mapping_end:
                out_ranges.append(Range(mapping[0], mapping[2] - (mapping_end - in_range_end)))
                open_ranges.append(Range(in_range.start, mapping[1] - in_range.start))

            elif in_range.start >= mapping[1] and in_range_end > mapping_end:
                offset = in_range.start - mapping[1]
                out_ranges.append(Range(mapping[0] + offset, mapping[2] - offset))
                open_ranges.append(Range(mapping[1] + mapping[2], (in_range_end - mapping_end)))

            else:
                offset = mapping[1] - in_range.start
                out_ranges.append(Range(mapping[0] + offset, mapping[2]))
                open_ranges.append(Range(in_range.start, offset))
                open_ranges.append(Range(mapping[1] + mapping[2], (in_range_end - mapping_end)))
        ranges_to_process = open_ranges

    if len(ranges_to_process) > 0:
        for r in ranges_to_process:
            out_ranges.append(r)

    return out_ranges

def solve_part_two(seeds: list[int], in_out_mappings: list[list[int]]) -> None:
    locations = []
    for i in range(0, len(seeds), 2):
        in_ranges = [Range(seeds[i], seeds[i + 1])]
        for in_out_mapping in in_out_mappings:
            in_ranges = get_mapped_ranges(in_ranges, in_out_mapping)

        for r in in_ranges:
            locations.append(r)

    print(min([r.start for r in locations]))


with open("../../../inputs/input05.txt", "r") as f:
    input_str = f.read()

    in_out_mappings: list[list[list[int]]] = []
    for i, line in enumerate(input_str.splitlines()):
        if line == "":
            continue
        if i == 0:
            seeds = [int(s) for s in line.split(":")[1][1:].split(" ")]
            continue

        if "map" in line:
            in_out_mappings.append([])
        else:
            in_out_mappings[-1].append([int(s) for s in line.split()])


solve_part_one(seeds, in_out_mappings)
solve_part_two(seeds, in_out_mappings)