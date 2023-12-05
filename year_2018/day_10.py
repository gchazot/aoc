import re
import unittest

from aoc_utils import test
from aoc_utils.data import data_lines


class TestSkyPoint(unittest.TestCase):
    def test_parses_input(self):
        point = SkyPoint.parse("position=< 9,  1> velocity=< 0,  2>")
        self._check_point(9, 1, 0, 2, point)

        negative_point = SkyPoint.parse("position=<-9, -1> velocity=<-0, -2>")
        self._check_point(-9, -1, 0, -2, negative_point)

    def test_after(self):
        point = SkyPoint.parse("position=< 9,  1> velocity=< 0,  2>")
        self._check_point(9, 3, 0, 2, point.after(1))

    def _check_point(self, x, y, u, v, point):
        self.assertEqual(x, point.x)
        self.assertEqual(y, point.y)
        self.assertEqual(u, point.u)
        self.assertEqual(v, point.v)


class SkyPoint:
    FORMAT_PATTERN = re.compile(
        r"position=< *(?P<x>-?\d+), *(?P<y>-?\d+)> velocity=< *(?P<u>-?\d+), *(?P<v>-?\d+)>"
    )

    @staticmethod
    def parse(point_string):
        match = SkyPoint.FORMAT_PATTERN.match(point_string)
        if match is None:
            print("Failed to parse: ", point_string)
        x = int(match.group('x'))
        y = int(match.group('y'))
        u = int(match.group('u'))
        v = int(match.group('v'))
        return SkyPoint(x, y, u, v)

    def __init__(self, x, y, u, v):
        self.x = x
        self.y = y
        self.u = u
        self.v = v

    def after(self, time):
        u = self.u
        v = self.v
        x = self.x + time * u
        y = self.y + time * v
        return SkyPoint(x, y, u, v)


class TestSkyMap(unittest.TestCase):
    def test_starts_empty(self):
        sky = SkyMap()
        self.assertEqual(0, sky.size())
        self.assertEqual([], list(sky.format()))

    def test_add(self):
        sky = SkyMap()
        sky.add(SkyPoint(0, 1, 2, 3))
        self.assertEqual(1, sky.size())
        sky.add(SkyPoint(4, 5, 6, 7))
        self.assertEqual(2, sky.size())

    def test_format(self):
        sky = SkyMap()

        sky.add(SkyPoint(0, 1, 2, 3))
        self.assertListEqual([
            "#",
        ], list(sky.format()))

        sky.add(SkyPoint(4, 5, 6, 7))
        self.assertListEqual([
            "#    ",
            "     ",
            "     ",
            "     ",
            "    #",
        ], list(sky.format()))

    def test_after(self):
        sky = SkyMap()
        sky.add(SkyPoint(0, 1, 2, 3))

        new_sky = sky.after(1)

        self.assertEqual(1, new_sky.size())

        point = new_sky._points[0]
        self.assertEqual(2, point.x)
        self.assertEqual(4, point.y)
        self.assertEqual(2, point.u)
        self.assertEqual(3, point.v)

    def test_score(self):
        sky = SkyMap()

        sky.add(SkyPoint(0, 0, -1, -1))
        self.assertEqual(0, sky.score())

        sky.add(SkyPoint(0, 2, -1, -1))
        self.assertEqual(0, sky.score())

        sky.add(SkyPoint(1, 0, -1, -1))
        self.assertEqual(1, sky.score())

        sky.add(SkyPoint(2, 0, -1, -1))
        self.assertEqual(2, sky.score())

        sky.add(SkyPoint(1, 1, -1, -1))
        self.assertEqual(3, sky.score())

    def test_surface(self):
        sky = SkyMap()
        self.assertEqual(0, sky.surface())

        sky.add(SkyPoint(0, 0, -1, -1))
        self.assertEqual(1, sky.surface())

        sky.add(SkyPoint(2, 2, -1, -1))
        self.assertEqual(9, sky.surface())

        sky.add(SkyPoint(3, 1, -1, -1))
        self.assertEqual(12, sky.surface())


class SkyMap:
    def __init__(self):
        self._points = []

    def size(self):
        return len(self._points)

    def add(self, point):
        self._points.append(point)

    def extend(self, points):
        self._points.extend(points)

    def after(self, time):
        sky = SkyMap()
        sky.extend(map(lambda p: p.after(time), self._points))
        return sky

    def format(self):
        for row in self._gen_table():
            chars = map(lambda v: "#" if v else " ", row)
            yield "".join(chars)

    def score(self):
        score = 0

        coordinates = self._get_coordinates()

        for x, y in coordinates:
            if (x+1, y) in coordinates:
                score += 1
            if (x, y+1) in coordinates:
                score += 1

        return score

    def surface(self):
        if len(self._points) == 0:
            return 0
        min_x, max_x, min_y, max_y = self._get_edges()
        return (max_x + 1 - min_x) * (max_y + 1 - min_y)

    def _gen_table(self):
        if self.size() == 0:
            return

        x_range, y_range = self._get_ranges()
        coordinates = self._get_coordinates()

        for y in y_range:
            row = [(x, y) in coordinates for x in x_range]
            yield row

    def _get_ranges(self):
        min_x, max_x, min_y, max_y = self._get_edges()
        x_range = range(min_x, max_x + 1)
        y_range = range(min_y, max_y + 1)
        return x_range, y_range

    def _get_edges(self):
        min_x = min(point.x for point in self._points)
        max_x = max(point.x for point in self._points)
        min_y = min(point.y for point in self._points)
        max_y = max(point.y for point in self._points)
        return min_x, max_x, min_y, max_y

    def _get_coordinates(self):
        return {(point.x, point.y) for point in self._points}


class TestSolver(unittest.TestCase):
    def test_deltas(self):
        solver = SkySolver(None, 9999)

        solver._surfaces = [1, 2, 3, 4]
        self.assertListEqual([1, 1, 1], list(solver.deltas()))

        solver._surfaces = [1, 2, 3, 1, 0, -4]
        self.assertListEqual([1, 1, -2, -1, -4], list(solver.deltas()))

    def test_finished(self):
        solver = SkySolver(None, sample_size=3)

        solver._surfaces = [10, 8, 6]
        self.assertFalse(solver.finished())

        solver._surfaces = [10, 8, 6, 4, 6]
        self.assertFalse(solver.finished())

        solver._surfaces = [10, 8, 6, 4, 6, 8]
        self.assertFalse(solver.finished())

        solver._surfaces = [10, 8, 6, 4, 6, 8, 10]
        self.assertTrue(solver.finished())

    def test_example(self):
        sky = SkyMap()
        for point_str in data_lines(2018, "day_10_example.txt"):
            point = SkyPoint.parse(point_str)
            sky.add(point)

        self.assertEqual(31, sky.size())

        solver = SkySolver(sky, sample_size=5)
        surface, iteration, table = solver.solve()

        self.assertEqual(80, surface)
        self.assertEqual(3, iteration)
        self.assertEqual([
            "#   #  ###",
            "#   #   # ",
            "#   #   # ",
            "#####   # ",
            "#   #   # ",
            "#   #   # ",
            "#   #   # ",
            "#   #  ###",
        ], list(table))

    @test.pypy_only("Taking too long")
    def test_mine(self):
        sky = SkyMap()
        for point_str in data_lines(2018, "day_10_mine.txt"):
            point = SkyPoint.parse(point_str)
            sky.add(point)

        self.assertEqual(328, sky.size())

        solver = SkySolver(sky, sample_size=10)
        surface, iteration, table = solver.solve()

        self.assertEqual(620, surface)
        self.assertEqual(10605, iteration)
        self.assertEqual([
            '#####   #    #     ###  #    #     ###    ##    ######  #    #',
            '#    #  #    #      #   #    #      #    #  #   #       #    #',
            '#    #   #  #       #    #  #       #   #    #  #        #  # ',
            '#    #   #  #       #    #  #       #   #    #  #        #  # ',
            '#####     ##        #     ##        #   #    #  #####     ##  ',
            '#    #    ##        #     ##        #   ######  #         ##  ',
            '#    #   #  #       #    #  #       #   #    #  #        #  # ',
            '#    #   #  #   #   #    #  #   #   #   #    #  #        #  # ',
            '#    #  #    #  #   #   #    #  #   #   #    #  #       #    #',
            '#####   #    #   ###    #    #   ###    #    #  ######  #    #',
        ], list(table))


class SkySolver:
    def __init__(self, sky_map, sample_size):
        self._sky = sky_map
        self._surfaces = []
        self._sample_size = sample_size

    def solve(self):
        t = 0
        while not self.finished():
            sky = self._sky.after(t)
            self._surfaces.append(sky.surface())
            t += 1
            if t > 100000:
                break
        iteration, surface = min(enumerate(self._surfaces), key=lambda i_surface: i_surface[1])
        best_sky = self._sky.after(iteration)

        return surface, iteration, best_sky.format()

    def finished(self):
        deltas = list(self.deltas(source=self._surfaces))
        if len(deltas) == 0:
            return False
        return all(delta > 0 for delta in deltas)

    def deltas(self, source=None):
        if source is None:
            source = self._surfaces
        end_index = len(source)
        start_index = max(1, end_index - self._sample_size)
        for i in range(start_index, end_index):
            yield source[i] - source[i-1]
