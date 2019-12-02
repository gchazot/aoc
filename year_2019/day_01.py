import unittest

from aoc_utils.data import data_lines


class TestRequiredFuel(unittest.TestCase):
    def test_examples_simple(self):
        self.assertEqual(2, required_fuel(12))
        self.assertEqual(2, required_fuel(14))
        self.assertEqual(654, required_fuel(1969))
        self.assertEqual(33583, required_fuel(100756))

    def test_mine_simple(self):
        self.assertEqual(
            3375962,
            required_fuel_overall(required_fuel, data_lines(2019, "day_01_mine.txt")),
        )

    def test_examples_total(self):
        self.assertEqual(2, required_total_fuel(14))
        self.assertEqual(966, required_total_fuel(1969))
        self.assertEqual(50346, required_total_fuel(100756))

    def test_mine_total(self):
        self.assertEqual(
            5061072,
            required_fuel_overall(required_total_fuel, data_lines(2019, "day_01_mine.txt")),
        )


def required_fuel_overall(fuel_func, lines):
    return sum(map(lambda line: fuel_func(int(line)), lines))


def required_total_fuel(mass):
    total_fuel = 0
    added_mass = mass
    while True:
        added_fuel = required_fuel(added_mass)
        if added_fuel <= 0:
            break
        total_fuel += added_fuel
        added_mass = added_fuel
    return total_fuel


def required_fuel(mass):
    return mass // 3 - 2
