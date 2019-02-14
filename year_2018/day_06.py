import collections
import unittest

from aoc_utils import data_file


class TestLandingArea(unittest.TestCase):
    def test_starts_empty(self):
        landing = WidestLandingArea()
        self.assertEqual(0, landing.size())

    def test_can_add_targets(self):
        landing = WidestLandingArea()
        landing.add('A', (1, 1))
        self.assertEqual(1, landing.size())
        landing.add('B', (1, 6))
        self.assertEqual(2, landing.size())
        landing.add('C', (8, 3))
        self.assertEqual(3, landing.size())

    def test_solve(self):
        landing = WidestLandingArea()
        landing.add('A', (1, 1))
        landing.add('B', (1, 6))
        landing.add('C', (8, 3))
        landing.add('D', (3, 4))
        landing.add('E', (5, 5))
        landing.add('F', (8, 9))

        self.assertEqual(('E', 17), landing.solve())

    @unittest.skip("Too slow")
    def test_solve_widest_mine(self):
        with open(data_file(2018, 'day_06_mine.txt')) as f:
            lines = f.readlines()
            pairs = [line.split(', ') for line in lines]
            coordss = [tuple(map(int, pair)) for pair in pairs]

        landing = WidestLandingArea()
        for i, coords in enumerate(coordss):
            landing.add(i, coords)

        self.assertEqual((29, 2342), landing.solve())

    def test_solve2(self):
        landing = ClosestLandingArea()
        landing.add('A', (1, 1))
        landing.add('B', (1, 6))
        landing.add('C', (8, 3))
        landing.add('D', (3, 4))
        landing.add('E', (5, 5))
        landing.add('F', (8, 9))

        self.assertEqual(16, landing.solve(max_distance=32))

    @unittest.skip("Too slow")
    def test_solve_closest_mine(self):
        with open(data_file(2018, 'day_06_mine.txt')) as f:
            lines = f.readlines()
            pairs = [line.split(', ') for line in lines]
            coordss = [tuple(map(int, pair)) for pair in pairs]

        landing = ClosestLandingArea()
        for i, coords in enumerate(coordss):
            landing.add(i, coords)

        self.assertEqual(43302, landing.solve())


def print_map(source_map, x_range, y_range):
    for y in y_range:
        row = [source_map.get((x, y), '.') or '=' for x in x_range]
        print(''.join(row))
    print()


class LandingArea(object):
    def __init__(self):
        self._targets = []
        self._map = {}
        self._top_left = [None, None]
        self._bottom_right = [None, None]

    def size(self):
        return len(self._targets)

    def add(self, i, coordinates):
        self._targets.append((i, coordinates))
        self._update_bounds(coordinates)

    def _distance(self, a, b):
        return abs(a[0] - b[0]) + abs(a[1] - b[1])

    def _each_point(self):
        for x in range(self._top_left[0], self._bottom_right[0] + 1):
            for y in range(self._top_left[1], self._bottom_right[1] + 1):
                yield x, y

    def _within_bounds(self, coordinates):
        return (self._top_left[0] <= coordinates[0] <= self._bottom_right[0]
                and self._top_left[1] <= coordinates[1] <= self._bottom_right[1])

    def _update_bounds(self, coordinates):
        if self._top_left[0] is None or self._top_left[0] > coordinates[0]:
            self._top_left[0] = coordinates[0]
        if self._top_left[1] is None or self._top_left[1] > coordinates[1]:
            self._top_left[1] = coordinates[1]
        if self._bottom_right[0] is None or self._bottom_right[0] < coordinates[0]:
            self._bottom_right[0] = coordinates[0]
        if self._bottom_right[1] is None or self._bottom_right[1] < coordinates[1]:
            self._bottom_right[1] = coordinates[1]


class WidestLandingArea(LandingArea):

    def solve(self):
        self._fill_map_of_closest()
        infinite_areas = self._infinite_areas()

        counts = collections.defaultdict(lambda: 0)

        for x, y in self._each_point():
            i = self._map[x, y]
            if i not in infinite_areas:
                counts[i] += 1

        return max(counts.items(), key=lambda i_count: i_count[1])

    def _infinite_areas(self):
        infinite = set()
        for x in range(self._top_left[0], self._bottom_right[0] + 1):
            top = (x, self._top_left[1])
            bottom = (x, self._bottom_right[1])
            infinite.add(self._map[top])
            infinite.add(self._map[bottom])
        for y in range(self._top_left[1], self._bottom_right[1] + 1):
            left = (self._top_left[0], y)
            right = (self._bottom_right[0], y)
            infinite.add(self._map[left])
            infinite.add(self._map[right])

        infinite.remove(None)

        return infinite

    def _fill_map_of_closest(self):
        for x, y in self._each_point():
            closest = None
            min_distance = None
            for i, coordinates in self._targets:
                distance = self._distance(coordinates, (x, y))
                if min_distance is None or min_distance > distance:
                    closest = [i]
                    min_distance = distance
                elif min_distance == distance:
                    closest.append(i)
            self._map[x, y] = closest[0] if len(closest) == 1 else None

    def _print(self):
        for y in range(self._top_left[1], self._bottom_right[1] + 1):
            row = [self._map.get((x, y), '.') or '=' for x in
                   (range(self._top_left[0], self._bottom_right[0] + 1))]
            print(''.join(row))
        print()


class ClosestLandingArea(LandingArea):
    def solve(self, max_distance=10000):
        self._fill_map_of_distances()

        count = 0
        for location in self._each_point():
            if self._map[location] < max_distance:
                count += 1
            else:
                self._map[location] = 0

        return count

    def _fill_map_of_distances(self):
        for current in self._each_point():
            self._map.setdefault(current, 0)
            for i, coordinates in self._targets:
                distance = self._distance(coordinates, current)
                self._map[current] += distance

    def _print(self):
        x_range = range(self._top_left[0], self._bottom_right[0] + 1)
        y_range = range(self._top_left[1], self._bottom_right[1] + 1)
        for y in y_range:
            row = ["{0:^#8}".format(self._map[x, y]) for x in x_range]
            print(''.join(row))
        print()
