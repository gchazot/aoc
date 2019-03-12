import itertools
import unittest

from aoc_utils.char_map import CharMap, MapExplorer, ProgressRules, ADJACENT_COORDINATES_DELTAS, \
    add_coordinates


class TestCoordinatesUtils(unittest.TestCase):
    def test_solve_tie(self):
        self.assertEqual(None, solve_tie([]))
        self.assertEqual((12, 34), solve_tie([(12, 34)]))
        self.assertEqual((1, 1), solve_tie([(1, 1), (2, 2)]))
        self.assertEqual((1, 1), solve_tie([(2, 2), (1, 1)]))
        self.assertEqual((2, 1), solve_tie([(1, 2), (2, 1)]))
        self.assertEqual((2, 1), solve_tie([(2, 1), (1, 2)]))


def solve_tie(options):
    if len(options):
        return sorted_by_priority(options)[0]


def sorted_by_priority(options):
    return sorted(options, key=reverse_coordinates)


def reverse_coordinates(coordinates):
    return tuple(i for i in reversed(coordinates))


class FindAllClosestRules(ProgressRules):
    def __init__(self, targets, allowed_values):
        super(FindAllClosestRules, self).__init__(allowed_values)
        self._targets = targets
        self._found_one = False
        self.results = []

    def stop_progressing(self):
        return self._found_one

    def examine(self, coordinates):
        if coordinates in self._targets:
            self._found_one = True
            self.results.append(coordinates)
            return False
        return True


class TestCaves(unittest.TestCase):
    def make_default_caves(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        return caves

    def test_init_fighters(self):
        caves = self.make_default_caves()
        fighters = caves.fighters

        self.assertSetEqual({'E', 'G'}, set(fighters.keys()))
        self.assertEqual({(1, 1): 200}, fighters['E'])
        self.assertEqual({(4, 1): 200, (2, 3): 200, (5, 3): 200}, fighters['G'])

    def test_get_targets(self):
        caves = self.make_default_caves()

        self.assertListEqual([(4, 1), (2, 3), (5, 3)], list(caves.get_targets("E")))
        self.assertListEqual([(1, 1)], list(caves.get_targets("G")))

    def test_get_in_range(self):
        caves = self.make_default_caves()

        self.assertListEqual([(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                             list(caves.get_in_range("E")))
        self.assertListEqual([(2, 1), (1, 2)],
                             list(caves.get_in_range("G")))

    def test_get_coordinates_around(self):
        caves = self.make_default_caves()
        self.assertListEqual([(2, 1), (1, 2)], list(caves.get_coordinates_around((1, 1))))
        self.assertListEqual([(3, 1), (5, 1)], list(caves.get_coordinates_around((4, 1))))
        self.assertListEqual([(2, 2), (1, 3), (3, 3)], list(caves.get_coordinates_around((2, 3))))
        self.assertListEqual([(5, 2)], list(caves.get_coordinates_around((5, 3))))

    def test_find_all_closest_rules(self):
        caves = Caves([
            "#######",
            "#E#.G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        finder = MapExplorer(caves._caves)
        rules = FindAllClosestRules(
            targets=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
            allowed_values=[EMPTY_VALUE]
        )
        finder.explore(start_point=(1, 1), rules=rules)
        self.assertListEqual([(2, 2), (1, 3)], list(rules.results))

    def test_iterate_units(self):
        caves = self.make_default_caves()
        self.assertListEqual([(1, 1), (4, 1), (2, 3), (5, 3)], caves._iterate_units())

    def test_get_attack_target(self):
        caves_2 = Caves([
            "#######",
            "#..EG.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        self.assertEqual((4, 1), caves_2.get_attack_target((3, 1), 'E'))
        self.assertEqual((3, 1), caves_2.get_attack_target((4, 1), 'G'))
        self.assertEqual(None, caves_2.get_attack_target((2, 3), 'G'))
        self.assertEqual(None, caves_2.get_attack_target((5, 3), 'G'))

    def test_find_next_destination(self):
        caves = self.make_default_caves()
        self.assertEqual((3, 1), caves.find_next_destination((1, 1), 'E'))
        self.assertEqual((2, 1), caves.find_next_destination((4, 1), 'G'))

        caves_2 = Caves([
            "#######",
            "#..EG.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        self.assertEqual((2, 2), caves_2.find_next_destination((3, 1), 'E'))
        self.assertEqual(None, caves_2.find_next_destination((4, 1), 'G'))


TEAMS = {'E', 'G'}
EMPTY_VALUE = '.'
WALL_VALUE = '#'


class Caves:
    def __init__(self, initial_map):
        self._caves = CharMap(input_lines=initial_map)
        self.fighters = {team: {} for team in TEAMS}
        for position, entry in self._caves.items():
            if entry in TEAMS:
                self.fighters[entry][position] = 200

    def get_attack_target(self, unit, team):
        for adjacent in self.get_coordinates_around(unit):
            if self._caves[adjacent] not in [EMPTY_VALUE, team]:
                return adjacent

    def find_next_destination(self, unit, team):
        in_range = self.get_in_range(team)
        if not in_range or unit in in_range:
            return None
        all_closest = self._find_all_closest(unit, in_range)
        return self._solve_tie(all_closest)

    def _iterate_units(self):
        all_units = itertools.chain.from_iterable(team.keys() for team in self.fighters.values())
        return sorted_by_priority(all_units)

    def get_coordinates_around(self, coordinates):
        for delta in ADJACENT_COORDINATES_DELTAS:
            adjacent = add_coordinates(coordinates, delta)
            if adjacent in self._caves and self._caves[adjacent] != WALL_VALUE:
                yield adjacent

    def get_in_range(self, opponent):
        in_range = []
        for target in self.get_targets(opponent):
            for coordinates in self.get_coordinates_around(target):
                if self._caves[coordinates] == EMPTY_VALUE:
                    in_range.append(coordinates)
        return sorted(in_range, key=lambda tup: (tup[1], tup[0]))

    def get_targets(self, opponent):
        for coordinates, entry in self._caves.items():
            if entry not in [WALL_VALUE, EMPTY_VALUE, opponent]:
                yield coordinates
