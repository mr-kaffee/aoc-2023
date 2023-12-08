import solution as sol


def test_exact_full_overlap():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 50, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == 100
    assert out_ranges[0].length == 10


def test_no_overlap_1():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 60, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == in_ranges[0].start
    assert out_ranges[0].length == in_ranges[0].length


def test_no_overlap_2():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 40, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == in_ranges[0].start
    assert out_ranges[0].length == in_ranges[0].length


def test_full_overlap_mapping_larger():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 50, 20]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == 100
    assert out_ranges[0].length == 10


def test_full_overlap_not_same_start():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 40, 20]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == 110
    assert out_ranges[0].length == 10


def test_full_overlap_inside_mapping():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 40, 30]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 1
    assert out_ranges[0].start == 110
    assert out_ranges[0].length == 10


def test_no_full_overlap_left_missing():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 55, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 2

    assert out_ranges[0].start == 100
    assert out_ranges[0].length == 5

    assert out_ranges[1].start == 50
    assert out_ranges[1].length == 5


def test_no_full_overlap_right_missing():
    in_ranges = [sol.Range(50, 10)]
    in_out_mapping = [[100, 45, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 2

    assert out_ranges[0].start == 105
    assert out_ranges[0].length == 5

    assert out_ranges[1].start == 55
    assert out_ranges[1].length == 5


def test_no_full_overlap_both_sides_missing():
    in_ranges = [sol.Range(50, 20)]
    in_out_mapping = [[100, 55, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mapping)

    assert len(out_ranges) == 3

    assert out_ranges[0].start == 105
    assert out_ranges[0].length == 10

    assert out_ranges[1].start == 50
    assert out_ranges[1].length == 5

    assert out_ranges[2].start == 65
    assert out_ranges[2].length == 5


def test_two_mapping_ranges():
    in_ranges = [sol.Range(50, 20)]
    in_out_mappings = [[100, 55, 10], [200, 65, 10]]

    out_ranges = sol.get_mapped_ranges(in_ranges, in_out_mappings)

    assert len(out_ranges) == 3

    assert out_ranges[0].start == 105
    assert out_ranges[0].length == 10

    assert out_ranges[1].start == 200
    assert out_ranges[1].length == 5

    assert out_ranges[2].start == 50
    assert out_ranges[2].length == 5
