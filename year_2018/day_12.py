from aoc_utils import data_lines
import unittest


class TestPlantation(unittest.TestCase):
    def test_starts_empty(self):
        pots = Plantation()
        self.assertEqual(0, len(pots))

    def test_init_state(self):
        pots = Plantation()
        pots.init_state("initial state: #..#.#..##......###...###")
        self.assertEqual(25, len(pots))
        self.assertEqual(11, pots.count())
        self.assertListEqual(list("#..#.#"), [pots[i] for i in range(6)])
        self.assertEqual("#", pots[0])
        self.assertEqual(".", pots[1])

    def test_parse_input_example(self):
        lines = data_lines(2018, "day_12_example.txt")
        pots, rules = parse_input(lines)

        self.assertEqual(25, len(pots))
        self.assertEqual(14, len(rules))

    def test_parse_input_mine(self):
        lines = data_lines(2018, "day_12_mine.txt")
        pots, rules = parse_input(lines)

        self.assertEqual(99, len(pots))
        self.assertEqual(20, len(rules))

    def test_iterate_without_rules(self):
        pots = Plantation()
        pots.init_state("initial state: #..#.#..##......###...###")
        rules = PlantationRules()
        pots.iterate(rules)

        self.assertEqual(0, len(pots))
        self.assertEqual(0, pots.count())

    def test_iterate_example(self):
        lines = data_lines(2018, "day_12_example.txt")
        pots, rules = parse_input(lines)

        pots.iterate(rules)
        self.check_pots(pots, 0, 25, 7, "#...#....#.....#..#..#..#")

        pots.iterate(rules)
        self.check_pots(pots, 0, 26, 11, "##..##...##....#..#..#..##")

        for _ in range(2, 10):
            pots.iterate(rules)
        self.check_pots(pots, 1, 31, 14, "#.#..#...#.##....##..##..##..##")

        for _ in range(10, 20):
            pots.iterate(rules)
        self.check_pots(pots, 2, 37, 19, "#....##....#####...#######....#.#..##")

    def test_iterate_mine(self):
        lines = data_lines(2018, "day_12_mine.txt")
        pots, rules = parse_input(lines)

        for _ in range(0, 20):
            pots.iterate(rules)
        self.check_pots(
            pots, 1, 120, 101,
            "#######.###.#####.#####.#.##.##.###.#.####.######.#.#######.#.####.##.##.###.##.###" +
            "#####################################")

    def check_pots(self, pots, offset, length, count, values):
        self.assertEqual(length, len(pots))
        self.assertEqual(count, pots.count())
        self.assertEqual(values, "".join(pots[i - offset] for i in range(length)))

    def test_sum_numbers_example(self):
        lines = data_lines(2018, "day_12_example.txt")
        pots, rules = parse_input(lines)
        for _ in range(0, 20):
            pots.iterate(rules)

        self.assertEqual(325, pots.sum_numbers())

    def test_sum_numbers_mine(self):
        lines = data_lines(2018, "day_12_mine.txt")
        pots, rules = parse_input(lines)
        for _ in range(0, 20):
            pots.iterate(rules)

        self.assertEqual(6201, pots.sum_numbers())

    def test_sum_numbers_mine_long(self):
        lines = data_lines(2018, "day_12_mine.txt")
        pots, rules = parse_input(lines)
        seen = {}
        for i in range(0, 92):
            pots.iterate(rules)
        # From here, all iterations are only shifting the whole plantation right
        pots._offset -= (50000000000 - 92)

        self.assertEqual(9300000001023, pots.sum_numbers())


def parse_input(lines):
    pots = Plantation()
    rules = PlantationRules()
    for i, line in enumerate(lines):
        if i == 0:
            pots.init_state(line)
        elif i > 1:
            rules.add(line)
    return pots, rules


class Plantation:
    def __init__(self):
        self._pots = ""
        self._offset = 0

    def __len__(self):
        return len(self._pots)

    def count(self):
        return self._pots.count("#")

    def init_state(self, init_line):
        self._pots = init_line[len("initial state: "):]

    def __getitem__(self, item):
        if not isinstance(item, int):
            raise NotImplementedError(type(item))
        actual_index = item + self._offset
        if actual_index < 0 or actual_index >= len(self._pots):
            raise IndexError("{} out or range".format(actual_index))
        return self._pots[actual_index]

    def iterate(self, rules):
        influence = 2
        block = 2 * influence + 1
        extra_pots = '.' * block
        pots = "{0}{1}{0}".format(extra_pots, self._pots)
        new_pots = ""
        num_blocks = len(pots) - 2 * influence
        for i in range(num_blocks):
            pattern = pots[i:i + block]
            if pattern in rules:
                new_pots += "#"
            else:
                new_pots += "."
        self._pots = new_pots.rstrip(".")
        self._offset += influence + 1
        self._clean_empty_pots()

    def sum_numbers(self):
        return sum(i - self._offset for i, pot in enumerate(self._pots) if pot == "#")

    def _clean_empty_pots(self):
        stripped_pots = self._pots.lstrip(".")
        removed_pots = len(self._pots) - len(stripped_pots)

        self._pots = stripped_pots
        self._offset -= removed_pots


class TestPlantationRules(unittest.TestCase):
    def test_starts_empty(self):
        rules = PlantationRules()
        self.assertEqual(0, len(rules))

    def test_add(self):
        rules = PlantationRules()
        rules.add("...## => #")
        self.assertEqual(1, len(rules))
        rules.add("..#.. => #")
        self.assertEqual(2, len(rules))

    def test_contains(self):
        rules = PlantationRules()

        self.assertFalse("...##" in rules)
        self.assertFalse("..#.." in rules)

        rules.add("...## => #")
        self.assertTrue("...##" in rules)
        self.assertFalse("..#.." in rules)

        rules.add("..#.. => #")
        self.assertTrue("...##" in rules)
        self.assertTrue("..#.." in rules)


class PlantationRules:
    def __init__(self):
        self._rules = set()

    def __len__(self):
        return len(self._rules)

    def __contains__(self, item):
        return item in self._rules

    def add(self, rule_line):
        pattern, result = rule_line.split(" => ")
        if result == "#":
            self._rules.add(pattern)
        elif result == '.':
            pass
        else:
            raise RuntimeError("Unknown pot state: '{}'".format(result))
