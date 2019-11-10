import re
import unittest

from aoc_utils.data import data_lines
from aoc_utils.geometry import add_coordinates, manhattan_distance


class TestEmergencyTeleporter(unittest.TestCase):
    def setUp(self):
        self.example_a = EmergencyTeleporter(suffix='example_a')
        self.example_b = EmergencyTeleporter(suffix='example_b')
        self.mine = EmergencyTeleporter(suffix='mine')

    def test_parses_bots(self):
        self.assertEqual([
            ((0, 0, 0), 4),
            ((1, 0, 0), 1),
            ((4, 0, 0), 3),
            ((0, 2, 0), 1),
            ((0, 5, 0), 3),
            ((0, 0, 3), 1),
            ((1, 1, 1), 1),
            ((1, 1, 2), 1),
            ((1, 3, 1), 1)
        ], self.example_a.bots)

    def test_find_strongest_example(self):
        self.assertEqual(((0, 0, 0), 4), self.example_a.find_strongest())

    def test_count_in_range_example(self):
        center, radius = self.example_a.find_strongest()
        in_range = list(self.example_a.find_in_range(center, radius))
        self.assertSetEqual({
            ((0, 0, 0), 4),
            ((1, 0, 0), 1),
            ((4, 0, 0), 3),
            ((0, 2, 0), 1),
            ((0, 0, 3), 1),
            ((1, 1, 1), 1),
            ((1, 1, 2), 1),
        }, set(in_range))
        self.assertEqual(7, len(in_range))

    def test_count_in_range_mine(self):
        strongest, strength = self.mine.find_strongest()
        in_range = list(self.mine.find_in_range(strongest, strength))
        self.assertEqual(906, len(in_range))

    def test_find_best_spot_example(self):
        best_spot, distance, count = self.example_b.best_spot()
        self.assertEqual((12, 12, 12), best_spot)
        self.assertEqual(36, distance)
        self.assertEqual(5, count)

    def test_mine(self):
        self.assertEqual(1000, len(self.mine.bots))
        self.assertEqual(99069470, max(rad for bot, rad in self.mine.bots))
        self.assertEqual(49576603, min(rad for bot, rad in self.mine.bots))

    @unittest.skip("I don't have a general solution for this")
    def test_find_best_spot_mine(self):
        best_spot, distance, count = self.mine.best_spot()
        self.assertEqual(121493971, distance)
        self.assertEqual((50995978, 21678597, 48819396), best_spot)
        self.assertEqual(985, count)


nanobot_template = re.compile(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)')


class EmergencyTeleporter:
    def __init__(self, suffix):
        self.bots = []
        for line in data_lines(2018, "day_23_{}.txt".format(suffix)):
            match = nanobot_template.match(line)
            groups = tuple(map(int, match.groups()))
            self.bots.append((groups[0:3], groups[3]))

    def find_strongest(self):
        return max(self.bots, key=lambda bot: bot[1])

    def find_in_range(self, center, radius):
        for position, strength in self.bots:
            if manhattan_distance(center, position) <= radius:
                yield position, strength

    def best_spot(self):
        dimension = len(self.bots[0][0])
        origin = tuple(0 for _ in range(dimension))

        corners = frozenset(self._generate_corners())
        overlaps = {
            corner: self._num_overlaps(corner)
            for corner in corners
        }
        best_corner_count = max(overlaps.values())
        best_corners = {
            corner: count
            for corner, count in overlaps.items()
            if count == best_corner_count
        }

        directions = [(1, 1, 0), (-1, 1, 0), (1, -1, 0), (-1, -1, 0), (1, 0, 1), (-1, 0, 1),
                      (1, 0, -1), (-1, 0, -1), (0, 1, 1), (0, -1, 1), (0, 1, -1), (0, -1, -1)]

        best_point = None
        best_point_distance = None
        best_point_count = None

        for point, count in best_corners.items():
            for direction in directions:
                current_point = point
                while True:
                    current_distance = manhattan_distance(origin, current_point)
                    current_count = self._num_overlaps(current_point)
                    if (
                            best_point is None or
                            current_count > best_point_count or
                            (current_count == best_point_count and current_distance < best_point_distance)
                    ):
                        best_point = current_point
                        best_point_count = current_count
                        best_point_distance = current_distance
                    elif current_count < best_point_count:
                        break

                    current_point = add_coordinates(current_point, direction)

        return best_point, best_point_distance, best_point_count

    def _generate_corners(self):
        any_point = self.bots[0][0]
        for bot, radius in self.bots:
            for i in any_point:
                for d in (-radius, radius):
                    delta = (d if j == i else 0 for j in any_point)
                    corner = add_coordinates(bot, delta)
                    yield(corner)

    def _num_overlaps(self, point):
        count = 0
        for position, strength in self.bots:
            if manhattan_distance(point, position) <= strength:
                count += 1
        return count
