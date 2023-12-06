from collections import defaultdict
import math
import unittest
try:
    from fractions import gcd
except ImportError:
    from math import gcd
from aoc_utils.char_map import CharMap
from aoc_utils.data import data_lines

ORIGIN = (0, 0)
ASTEROID = "#"
EMPTY = '.'


class TestAsteroidMap(unittest.TestCase):
    def test_relative_coordinates(self):
        for coordinates in [(1, 0), (4, 0), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (4, 3), (3, 4), (4, 4)]:
            self.assertEqual(
                coordinates,
                relative_coordinates(coordinates, ORIGIN)
            )
            self.assertEqual(
                tuple(-1 * coordinate for coordinate in coordinates),
                relative_coordinates(ORIGIN, coordinates)
            )

        self.assertEqual((1, 0), relative_coordinates((2, 0), (1, 0)))
        self.assertEqual((0, 1), relative_coordinates((0, 2), (0, 1)))
        self.assertEqual((1, 1), relative_coordinates((2, 2), (1, 1)))
        self.assertEqual((-1, 0), relative_coordinates((1, 0), (2, 0)))
        self.assertEqual((0, -1), relative_coordinates((0, 1), (0, 2)))
        self.assertEqual((-1, -1), relative_coordinates((1, 1), (2, 2)))

        self.assertEqual((2, -3), relative_coordinates((7, 3), (5, 6)))
        self.assertEqual((-4, 5), relative_coordinates((1, 11), (5, 6)))

    def test_get_asteroid_coordinates(self):
        asteroids = AsteroidMap([
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##",
        ])
        self.assertEqual(
            [(1, 0), (4, 0), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (4, 3), (3, 4), (4, 4)],
            list(asteroids.asteroid_coordinates()),
        )
        self.assertEqual(
            [(-2, -4), (1, -4), (-3, -2), (-2, -2), (-1, -2), (0, -2), (1, -2), (1, -1), (0, 0), (1, 0)],
            list(asteroids.asteroid_coordinates(relative_to=(3, 4))),
        )

    def test_count_in_sight(self):
        asteroids = AsteroidMap([
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##",
        ])
        self.assertEqual(
            ".7..7"
            "....."
            "67775"
            "....7"
            "...87"
            ,
            "".join(
                str(asteroids.count_in_sight(coordinates)) if content == ASTEROID else "."
                for coordinates, content in asteroids.items()
            )
        )

    def test_best_location(self):
        asteroids_1 = AsteroidMap([
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##",
        ])
        self.assertEqual(
            ((3, 4), 8),
            asteroids_1.best_survey_location(),
        )

        asteroids_2 = AsteroidMap([
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ])
        self.assertEqual(
            ((5, 8), 33),
            asteroids_2.best_survey_location(),
        )

        asteroids_3 = AsteroidMap([
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ])
        self.assertEqual(
            ((1, 2), 35),
            asteroids_3.best_survey_location(),
        )

        asteroids_4 = AsteroidMap([
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ])
        self.assertEqual(
            ((6, 3), 41),
            asteroids_4.best_survey_location(),
        )

    def test_best_location_example5(self):
        asteroids_5 = AsteroidMap([
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ])
        self.assertEqual(
            ((11, 13), 210),
            asteroids_5.best_survey_location(),
        )

    def test_best_location_mine(self):
        asteroids = AsteroidMap(data_lines(2019, "day_10_mine.txt"))
        self.assertEqual(
            ((23, 20), 334),
            asteroids.best_survey_location(),
        )

    def test_heading(self):
        self.assertEqual(0, heading((0, -1)))
        self.assertEqual(90, heading((1, 0)))
        self.assertEqual(180, heading((0, 1)))
        self.assertEqual(270, heading((-1, 0)))

        self.assertEqual(45, heading((1, -1)))
        self.assertEqual(135, heading((1, 1)))
        self.assertEqual(225, heading((-1, 1)))
        self.assertEqual(315, heading((-1, -1)))

    def test_order_laser_targets(self):
        asteroids = AsteroidMap([
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....X...###..",
            "..#.#.....#....##",
        ])
        vaporised_order = list(asteroids.order_laser_targets((8, 3)))
        self.assertEqual((8, 1), vaporised_order.pop(0))
        self.assertEqual((9, 0), vaporised_order.pop(0))
        self.assertEqual((9, 1), vaporised_order.pop(0))
        self.assertEqual((10, 0), vaporised_order.pop(0))
        self.assertEqual((9, 2), vaporised_order.pop(0))
        self.assertEqual((11, 1), vaporised_order.pop(0))
        self.assertEqual((12, 1), vaporised_order.pop(0))
        self.assertEqual((11, 2), vaporised_order.pop(0))
        self.assertEqual((15, 1), vaporised_order.pop(0))

        self.assertEqual((12, 2), vaporised_order.pop(0))
        self.assertEqual((13, 2), vaporised_order.pop(0))
        self.assertEqual((14, 2), vaporised_order.pop(0))
        self.assertEqual((15, 2), vaporised_order.pop(0))
        self.assertEqual((12, 3), vaporised_order.pop(0))
        self.assertEqual((16, 4), vaporised_order.pop(0))
        self.assertEqual((15, 4), vaporised_order.pop(0))
        self.assertEqual((10, 4), vaporised_order.pop(0))
        self.assertEqual((4, 4), vaporised_order.pop(0))

    def test_order_laser_targets_example5(self):
        asteroids = AsteroidMap([
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ])
        vaporised_order = list(asteroids.order_laser_targets((11, 13)))
        self.assertEqual((11, 12), vaporised_order[0])  # 1st
        self.assertEqual((12, 1), vaporised_order[1])  # 2nd
        self.assertEqual((12, 2), vaporised_order[2])  # 3rd
        self.assertEqual((12, 8), vaporised_order[9])  # 10th
        self.assertEqual((16, 0), vaporised_order[19])  # 20th
        self.assertEqual((16, 9), vaporised_order[49])  # 50th
        self.assertEqual((10, 16), vaporised_order[99])  # 100th
        self.assertEqual((9, 6), vaporised_order[198])  # 199tht
        self.assertEqual((8, 2), vaporised_order[199])  # 200tht
        self.assertEqual((10, 9), vaporised_order[200])  # 201st
        self.assertEqual((11, 1), vaporised_order[298])  # 299th

    def test_order_laser_targets_mine(self):
        asteroids = AsteroidMap(data_lines(2019, "day_10_mine.txt"))
        vaporised_order = list(asteroids.order_laser_targets((23, 20)))
        self.assertEqual((11, 19), vaporised_order[199])  # 200th

        x, y = vaporised_order[199]
        self.assertEqual(1119, x * 100 + y)


class AsteroidMap(CharMap):
    def __init__(self, input_lines):
        super(AsteroidMap, self).__init__(input_lines)

    def asteroid_coordinates(self, relative_to=ORIGIN):
        for position, value in self.items():
            if value == ASTEROID:
                yield relative_coordinates(position, relative_to)

    def count_in_sight(self, center):
        other_coords = self.other_polar_coords(center)
        directions = {direction for _other, direction, _distance in other_coords}
        return len(directions)

    def best_survey_location(self):
        visibles = self.visible_asteroids_counts()
        return max(visibles.items(), key=lambda pair: pair[1])

    def order_laser_targets(self, center):
        targets = self.other_polar_coords(center)
        by_direction = defaultdict(dict)
        for other, direction, distance in targets:
            by_direction[direction][other] = distance

        by_direction_sorted = {}
        for direction, direction_targets in by_direction.items():
            sorted_other_distance = sorted(direction_targets.keys(), key=lambda other: direction_targets[other])
            by_direction_sorted[direction] = list(sorted_other_distance)

        while by_direction_sorted:
            sorted_directions = sorted(by_direction_sorted.keys(), key=heading)
            for direction in sorted_directions:
                vaporise = by_direction_sorted[direction].pop(0)
                if not by_direction_sorted[direction]:
                    by_direction_sorted.pop(direction)
                yield vaporise

    def visible_asteroids_counts(self):
        relatives = self.relative_polar_coords()
        seen_directions = defaultdict(set)
        for center, _other, direction, _distance in relatives:
            seen_directions.setdefault(center, set())
            seen_directions[center].add(direction)

        return {
            center: len(directions)
            for center, directions in seen_directions.items()
        }

    def relative_polar_coords(self):
        for center in self.asteroid_coordinates():
            for other, direction, distance in self.other_polar_coords(center):
                yield center, other, direction, distance

    def other_polar_coords(self, center):
        for other in self.asteroid_coordinates():
            if center == other:
                continue

            direction, distance = polar_coords(center, other)

            yield other, direction, distance


def polar_coords(center, other):
    relative = relative_coordinates(other, center)
    direction = simplify_coords(relative)
    distance = relative[0] ** 2 + relative[1] ** 2
    return direction, distance


def relative_coordinates(coordinates, center):
    return tuple(a-b for a, b in zip(coordinates, center))


def simplify_coords(coordinates):
    x, y = coordinates
    if x == 0 and y == 0:
        raise RuntimeError
    elif x == 0:
        return 0, 1 if y > 0 else -1
    elif y == 0:
        return 1 if x > 0 else -1, 0
    else:
        cd = abs(gcd(x, y))
        return x // cd, y // cd


def heading(coordinates):
    x, y = coordinates
    return (450 - math.degrees(math.atan2(-y, x))) % 360
