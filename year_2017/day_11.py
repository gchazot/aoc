from unittest import TestCase
import operator
from aoc_utils.data import data_text


class Coordinates(object):
    def __init__(self, coordinates):
        self.coordinates = coordinates[:]

    def __str__(self):
        return "{}".format(self.coordinates)

    def __getitem__(self, item):
        return self.coordinates[item]

    def __len__(self):
        return len(self.coordinates)

    def __add__(self, other):
        new_coordinates = map(operator.add, self.coordinates, other.coordinates)
        return Coordinates(new_coordinates)

    def __iadd__(self, other):
        self.coordinates = list(map(operator.add, self.coordinates, other.coordinates))
        return self

    def __eq__(self, other):
        return all(map(operator.eq, self.coordinates, other.coordinates))


directions = {
    "n": Coordinates([0, 1, 1]),
    "s": Coordinates([0, -1, -1]),
    "ne": Coordinates([1, 1, 0]),
    "se": Coordinates([1, 0, -1]),
    "nw": Coordinates([-1, 0, 1]),
    "sw": Coordinates([-1, -1, 0])
}


class GridWalker:
    def __init__(self):
        self.position = Coordinates([0, 0, 0])
        self.furthest = 0

    def walk(self, route):
        for coord in route.split(","):
            step = directions[coord]
            self.position += step
            self.furthest = max(self.furthest, self.distance())

    def distance(self):
        return max(list(map(abs, self.position.coordinates)) + [0])


def final_coordinates(route):
    walker = GridWalker()
    walker.walk(route)
    return walker.position


def shortest_distance(route):
    walker = GridWalker()
    walker.walk(route)
    return walker.distance()


class TestHexWalk(TestCase):
    def setUp(self):
        def assert_coordinates_equal(a, b, msg=None):
            if msg is None:
                msg = "Expected: {}\nActual: {}".format(a, b)
            if not a == b:
                raise self.failureException(msg)

        self.addTypeEqualityFunc(Coordinates, assert_coordinates_equal)

    def test_final_coordinates(self):
        self.assertEqual(Coordinates([3, 3, 0]), final_coordinates("ne,ne,ne"))
        self.assertEqual(Coordinates([0, 0, 0]), final_coordinates("ne,ne,sw,sw"))
        self.assertEqual(Coordinates([2, 0, -2]), final_coordinates("ne,ne,s,s"))
        self.assertEqual(Coordinates([-1, -3, -2]), final_coordinates("se,sw,se,sw,sw"))

    def test_hexample(self):
        self.assertEqual(3, shortest_distance("ne,ne,ne"))
        self.assertEqual(0, shortest_distance("ne,ne,sw,sw"))
        self.assertEqual(2, shortest_distance("ne,ne,s,s"))
        self.assertEqual(3, shortest_distance("se,sw,se,sw,sw"))

    def test_mine(self):
        file_text = data_text(2017, "day_11_mine.txt")
        walker = GridWalker()
        walker.walk(file_text)
        self.assertEqual(687, walker.distance())
        self.assertEqual(1483, walker.furthest)