from unittest import TestCase
from aoc_utils.data import data_file


def check_triangle(sides):
    total = 0
    longest = 0
    for side in sides:
        total += side
        longest = max(longest, side)
    return total - 2 * longest > 0


def make_triangles_rows(integer_rows):
    return integer_rows


def make_triangles_columns(integer_rows):
    triangles = [[], [], []]
    for line_number, integers in enumerate(integer_rows, start=1):
        integers_list = list(integers)
        triangles[0].append(integers_list[0])
        triangles[1].append(integers_list[1])
        triangles[2].append(integers_list[2])
        if line_number % 3 == 0:
            yield triangles[0]
            yield triangles[1]
            yield triangles[2]
            triangles = [[], [], []]


def check_all_triangles(filename, make_triangles):
    with open(filename) as f:
        int_rows = map(lambda line: map(int, line.split()), f.readlines())
        triangles = make_triangles(int_rows)
        return sum(map(check_triangle, triangles))


class TestTriangles(TestCase):
    my_file = data_file(2016, "day_03_mine.txt")

    def test_1_example(self):
        self.assertFalse(check_triangle([5, 10, 25]))
        self.assertFalse(check_triangle([10, 5, 25]))
        self.assertFalse(check_triangle([10, 25, 5]))
        self.assertFalse(check_triangle([25, 10, 5]))
        self.assertFalse(check_triangle([5, 25, 10]))
        self.assertFalse(check_triangle([25, 5, 10]))

        self.assertFalse(check_triangle(map(int, "  139  444  640".split())))
        self.assertTrue(check_triangle(map(int, "  171  757  815".split())))

    def test_1_mine(self):
        self.assertEqual(993, check_all_triangles(self.my_file, make_triangles_rows))

    def test_2_example(self):
        example_file = data_file(2016, "day_03_2_example.txt")
        self.assertEqual(4, check_all_triangles(example_file, make_triangles_columns))

    def test_2_mine(self):
        self.assertEqual(1849, check_all_triangles(self.my_file, make_triangles_columns))

