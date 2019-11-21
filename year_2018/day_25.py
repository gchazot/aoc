import unittest

from aoc_utils.data import data_lines
from aoc_utils.geometry import manhattan_distance

examples = [
    [
        "0,0,0,0",
        "3,0,0,0",
        "0,3,0,0",
        "0,0,3,0",
        "0,0,0,3",
        "0,0,0,6",
        "9,0,0,0",
        "12,0,0,0",
    ],
    [
        "-1,2,2,0",
        "0,0,2,-2",
        "0,0,0,-2",
        "-1,2,0,0",
        "-2,-2,-2,2",
        "3,0,2,-1",
        "-1,3,2,2",
        "-1,0,-1,0",
        "0,2,1,-2",
        "3,0,0,0",
    ],
    [
        "1,-1,0,1",
        "2,0,-1,0",
        "3,2,-1,0",
        "0,0,3,1",
        "0,0,-1,-1",
        "2,3,-2,0",
        "-2,2,0,0",
        "2,-2,0,-1",
        "1,-1,0,-1",
        "3,2,0,2",
    ],
    [
        "1,-1,-1,-2",
        "-2,-2,0,1",
        "0,2,1,3",
        "-2,3,-2,1",
        "0,2,3,-2",
        "-1,-1,1,-2",
        "0,-2,-1,0",
        "-2,2,3,-1",
        "1,2,2,0",
        "-1,-2,0,-2",
    ]
]


class TestConstellations(unittest.TestCase):
    def test_find_connections(self):
        example_1 = parse(examples[0])
        start_1 = parse_point(examples[0][0])
        example_1.remove(start_1)
        self.assertSetEqual(
            {(3, 0, 0, 0), (0, 3, 0, 0), (0, 0, 3, 0), (0, 0, 0, 3)},
            find_connections(example_1, start_1),
        )

        example_2 = parse(examples[1])
        start_2 = parse_point(examples[1][0])
        example_2.remove(start_2)
        self.assertSetEqual(
            {(-1, 2, 0, 0), (-1, 3, 2, 2)},
            find_connections(example_2, start_2),
        )

    def test_find_constellation(self):
        example_1 = parse(examples[0])
        start_1 = (0, 0, 0, 0)
        example_1.remove(start_1)
        constellation_1 = find_constellation(example_1, start_1)
        self.assertSetEqual(
            {(0, 0, 0, 0), (3, 0, 0, 0), (0, 3, 0, 0), (0, 0, 3, 0), (0, 0, 0, 3), (0, 0, 0, 6)},
            constellation_1,
        )
        self.assertSetEqual(
            {(9, 0, 0, 0), (12, 0, 0, 0)},
            example_1,
        )

        example_2 = parse(examples[1])
        start_2 = (-1, 2, 2, 0)
        example_2.remove(start_2)
        constellation_2 = find_constellation(example_2, start_2)
        self.assertSetEqual(
            {(-1, 2, 2, 0), (-1, 2, 0, 0), (-1, 3, 2, 2), (-1, 0, -1, 0)},
            constellation_2,
        )
        self.assertSetEqual({
                (3, 0, 0, 0), (0, 2, 1, -2), (-2, -2, -2, 2),
                (0, 0, 0, -2), (3, 0, 2, -1), (0, 0, 2, -2),
            },
            example_2,
        )
        start_2 = (3, 0, 0, 0)
        example_2.remove(start_2)
        constellation_2 = find_constellation(example_2, start_2)
        self.assertSetEqual(
            {(3, 0, 0, 0), (3, 0, 2, -1)},
            constellation_2,
        )
        start_2 = (0, 2, 1, -2)
        example_2.remove(start_2)
        constellation_2 = find_constellation(example_2, start_2)
        self.assertSetEqual(
            {(0, 2, 1, -2), (0, 0, 0, -2), (0, 0, 2, -2)},
            constellation_2,
        )
        self.assertSetEqual({(-2, -2, -2, 2)}, example_2)

    def test_find_all_constellations(self):
        expected_counts = [2, 4, 3, 8]
        for i, example in enumerate(examples):
            points = parse(examples[i])
            constellations = list(find_all_constellations(points))
            self.assertEqual(expected_counts[i], len(constellations))

        my_lines = data_lines(2018, "day_25_mine.txt")
        my_points = parse(my_lines)
        my_constellations = list(find_all_constellations(my_points))
        self.assertEqual(420, len(my_constellations))


def find_all_constellations(points):
    while len(points) > 0:
        start = points.pop()
        yield find_constellation(points, start)


def find_constellation(points, constellation):
    result = set()
    wave = {constellation}
    while True:
        additions = set()
        for wave_point in wave:
            connections = find_connections(points, wave_point)
            additions.update(connections)
            points.difference_update(additions)
        result.update(wave)
        if len(additions) == 0:
            break
        wave = additions
    return result


def find_connections(points, start):
    return {point for point in points if manhattan_distance(start, point) <= 3}


class TestParse(unittest.TestCase):
    def check_points(self, expected_size, points):
        self.assertEqual(expected_size, len(points))
        self.assertTrue(all(len(point) == 4 for point in points))
        self.assertTrue(
            all(
                all(isinstance(coordinate, int) for coordinate in point)
            ) for point in points
        )

    def test_examples(self):
        for i, example in enumerate(examples):
            size = 8 if i == 0 else 10
            self.check_points(size, parse(example))

    def test_mine(self):
        my_lines = data_lines(2018, "day_25_mine.txt")
        self.check_points(1166, parse(my_lines))


def parse(lines):
    return set(parse_lines(lines))


def parse_lines(lines):
    for line in lines:
        yield parse_point(line)


def parse_point(line):
    return tuple(map(int, line.split(',')))
