import itertools
import unittest

from aoc_utils.char_map import CharMap, MapExplorer, ProgressRules, ADJACENT_COORDINATES_DELTAS, \
    add_coordinates


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

    def test_find_all_closest(self):
        caves = Caves([
            "#######",
            "#E#.G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual(
            [(2, 2), (1, 3)],
            list(
                caves._find_all_closest(
                    from_coords=(1, 1),
                    targets=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)]
                )
            )
        )

    def test_solve_tie(self):
        self.assertEqual(None, Caves._solve_tie([]))
        self.assertEqual((12, 34), Caves._solve_tie([(12, 34)]))
        self.assertEqual((1, 1), Caves._solve_tie([(1, 1), (2, 2)]))
        self.assertEqual((1, 1), Caves._solve_tie([(2, 2), (1, 1)]))
        self.assertEqual((2, 1), Caves._solve_tie([(1, 2), (2, 1)]))
        self.assertEqual((2, 1), Caves._solve_tie([(2, 1), (1, 2)]))

    def test_iterate_units(self):
        caves = self.make_default_caves()
        self.assertListEqual([(1, 1), (4, 1), (2, 3), (5, 3)], caves._iterate_units())

    def test_find_next_target(self):
        caves = self.make_default_caves()

        self.assertEqual((3, 1), caves._find_next_target((1, 1), 'E'))


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

    def _find_next_target(self, unit, team):
        targets = self.get_targets(team)
        all_closest = self._find_all_closest(unit, targets)
        return self._solve_tie(all_closest)

    def _find_all_closest(self, from_coords, targets):
        finder = MapExplorer(self._caves)
        rules = FindAllClosestRules(targets, [EMPTY_VALUE])
        finder.explore(from_coords, rules)
        return rules.results

    def _iterate_units(self):
        all_units = itertools.chain.from_iterable(team.keys() for team in self.fighters.values())
        return self._sorted_by_priority(all_units)

    @staticmethod
    def _solve_tie(options):
        if len(options):
            return Caves._sorted_by_priority(options)[0]

    @staticmethod
    def _sorted_by_priority(options):
        return sorted(options, key=Caves._reverse_coordinates)

    @staticmethod
    def _reverse_coordinates(coordinates):
        return tuple(i for i in reversed(coordinates))

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
