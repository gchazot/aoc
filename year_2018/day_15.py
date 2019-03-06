import unittest

from aoc_utils.char_map import CharMap, MapExplorer, ProgressRules


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
    def test_get_targets(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual([(4, 1), (2, 3), (5, 3)], list(caves.get_targets("E")))
        self.assertListEqual([(1, 1)], list(caves.get_targets("G")))

    def test_get_in_range(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual([(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                             list(caves.get_in_range("E")))

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
            list(caves._find_all_closest(
                from_coords=(1, 1),
                targets=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                allowed_values=["."])
            )
        )

    def test_solve_tie(self):
        self.assertEqual(None, Caves._solve_tie([]))
        self.assertEqual((12, 34), Caves._solve_tie([(12, 34)]))
        self.assertEqual((1, 1), Caves._solve_tie([(1, 1), (2, 2)]))
        self.assertEqual((1, 1), Caves._solve_tie([(2, 2), (1, 1)]))
        self.assertEqual((1, 2), Caves._solve_tie([(1, 2), (2, 1)]))
        self.assertEqual((1, 2), Caves._solve_tie([(2, 1), (1, 2)]))


class Caves:
    def __init__(self, initial_map):
        self._caves = CharMap(input_lines=initial_map)

    def _find_all_closest(self, from_coords, targets, allowed_values):
        finder = MapExplorer(self._caves)
        rules = FindAllClosestRules(targets, allowed_values)
        finder.explore(from_coords, rules)
        closest = rules.results
        return closest

    @staticmethod
    def _solve_tie(potential_targets):
        if len(potential_targets):
            return sorted(potential_targets)[0]

    def get_in_range(self, opponent):
        in_range = []
        for x, y in self.get_targets(opponent):
            for i, j in ((0, -1), (-1, 0), (1, 0), (0, 1)):
                try:
                    u = x + i
                    v = y + j
                    value = self._caves[u, v]
                except IndexError:
                    continue
                else:
                    if value == '.':
                        in_range.append((u, v))
        return sorted(in_range, key=lambda tup: (tup[1], tup[0]))

    def get_targets(self, opponent):
        for cordinates, entry in self._caves.items():
            if entry not in ['#', '.', opponent]:
                yield cordinates
