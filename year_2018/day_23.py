import re
import unittest

from aoc_utils.data import data_lines
from aoc_utils.geometry import manhattan_distance


class TestEmergencyTeleporter(unittest.TestCase):
    def setUp(self):
        self.example_a = EmergencyTeleporter(suffix='example_a')
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
