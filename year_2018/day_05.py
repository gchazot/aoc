from aoc_utils import data_file

import unittest


class TestPolymerReduce(unittest.TestCase):
    def test_is_opposite_case(self):
        self.assertTrue(is_opposite_case("a", "A"))
        self.assertTrue(is_opposite_case("A", "a"))
        self.assertTrue(is_opposite_case("b", "B"))
        self.assertTrue(is_opposite_case("B", "b"))
        self.assertFalse(is_opposite_case("a", "b"))
        self.assertFalse(is_opposite_case("A", "B"))
        self.assertFalse(is_opposite_case("a", "B"))
        self.assertFalse(is_opposite_case("A", "b"))

    def test_simple(self):
        self.assertEqual("", polymer_reduce("aA"))
        self.assertEqual("", polymer_reduce("Aa"))
        self.assertEqual("aa", polymer_reduce("aa"))
        self.assertEqual("ab", polymer_reduce("ab"))

    def test_nested(self):
        self.assertEqual("", polymer_reduce("abBA"))
        self.assertEqual("abAB", polymer_reduce("abAB"))

    def test_complex(self):
        self.assertEqual("dabCBAcaDA", polymer_reduce("dabAcCaCBAcCcaDA"))

    def test_mine(self):
        with open(data_file(2018, "day_05_mine.txt")) as f:
            polymer = f.read()
            reduced = polymer_reduce(polymer)
            self.assertEqual(10584, len(reduced))


def is_opposite_case(letter1, letter2):
    return abs(ord(letter1) - ord(letter2)) == ord("a") - ord("A")


def polymer_reduce(polymer):
    poly_list = [char for char in polymer]
    current = 0
    while True:
        if current >= len(poly_list) - 1:
            break
        if is_opposite_case(poly_list[current], poly_list[current + 1]):
            del poly_list[current]
            del poly_list[current]
            current = max(0, current - 1)
            continue
        current += 1
    return "".join(poly_list)
