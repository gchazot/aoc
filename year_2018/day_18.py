from __future__ import print_function
from collections import Counter
import unittest

from aoc_utils.char_map import CharMap, add_coordinates
from aoc_utils.data import data_lines


class TestLumberJacks(unittest.TestCase):
    @staticmethod
    def example_woodland():
        lines = data_lines(2018, "day_18_example.txt")
        return Woodlands(input_lines=lines)

    def test_counts_around(self):
        woodland = self.example_woodland()

        self.assertEqual({
            OPEN: 2,
            TREES: 0,
            LUMBER: 1,
        }, woodland.counts_around((0, 0)))

    def test_next_status(self):
        self.assertEqual(OPEN, Woodlands.next_state(OPEN, Counter({})))
        self.assertEqual(OPEN, Woodlands.next_state(OPEN, Counter({TREES: 2})))
        self.assertEqual(TREES, Woodlands.next_state(OPEN, Counter({TREES: 3})))

        self.assertEqual(TREES, Woodlands.next_state(TREES, Counter({})))
        self.assertEqual(TREES, Woodlands.next_state(TREES, Counter({LUMBER: 2})))
        self.assertEqual(LUMBER, Woodlands.next_state(TREES, Counter({LUMBER: 3})))

        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({})))
        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({OPEN: 1})))
        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({TREES: 1})))
        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({LUMBER: 1})))
        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({OPEN: 1, TREES: 1})))
        self.assertEqual(OPEN, Woodlands.next_state(LUMBER, Counter({OPEN: 1, LUMBER: 1})))
        self.assertEqual(LUMBER, Woodlands.next_state(LUMBER, Counter({TREES: 1, LUMBER: 1})))

    def test_iterate(self):
        woodland = self.example_woodland()

        woodland.iterate()
        self.assertListEqual([
            '.......##.',
            '......|###',
            '.|..|...#.',
            '..|#||...#',
            '..##||.|#|',
            '...#||||..',
            '||...|||..',
            '|||||.||.|',
            '||||||||||',
            '....||..|.',
        ], list(woodland.lines()))

    def test_resource_value(self):
        woodland = self.example_woodland()

        for _ in range(10):
            woodland.iterate()

        self.assertEqual(1147, woodland.resource_value())

    def test_resource_value_mine(self):
        lines = data_lines(2018, "day_18_mine.txt")
        woodland = Woodlands(input_lines=lines)

        for _ in range(10):
            woodland.iterate()

        self.assertEqual(678529, woodland.resource_value())

    def test_resource_value_mine_loooong(self):
        lines = data_lines(2018, "day_18_mine.txt")
        woodland = Woodlands(input_lines=lines)

        seen = {}
        i = 0
        while i < 1000000000:
            key = woodland._data.tobytes()
            if key in seen:
                cycle = i - seen[key]
                i += cycle * ((1000000000 - i) // cycle)
            else:
                seen[key] = i

            woodland.iterate()
            i += 1

        self.assertEqual(224005, woodland.resource_value())


OPEN = '.'
TREES = '|'
LUMBER = '#'
ADJACENT_DELTAS = [(u, v) for u in range(-1, 2) for v in range(-1, 2) if not (u == 0 and v == 0)]


class Woodlands(CharMap):
    def __init__(self, input_lines):
        super(Woodlands, self).__init__(input_lines=input_lines)

    def counts_around(self, coordinates):
        counts = Counter({content: 0 for content in (OPEN, TREES, LUMBER)})
        for delta in ADJACENT_DELTAS:
            try:
                value = self[add_coordinates(coordinates, delta)]
                counts[value] += 1
            except IndexError:
                pass

        return counts

    @staticmethod
    def next_state(state, counts):
        if state == OPEN and counts[TREES] >= 3:
            return TREES
        elif state == TREES and counts[LUMBER] >= 3:
            return LUMBER
        elif state == LUMBER and (counts[TREES] < 1 or counts[LUMBER] < 1):
            return OPEN
        return state

    def iterate(self):
        new_state = CharMap(width_height=(self.width, self.height))

        for coordinates, state in self.items():
            counts = self.counts_around(coordinates)
            next_state = self.next_state(state, counts)
            new_state[coordinates] = next_state

        self.swap(new_state)

    def resource_value(self):
        counts = self.counts()
        return counts[TREES] * counts[LUMBER]

    def counts(self):
        return Counter(self.values())
