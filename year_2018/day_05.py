from aoc_utils.data import data_text

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

    def test_reduce_mine(self):
        polymer = data_text(2018, "day_05_mine.txt")
        reduced = polymer_reduce(polymer)
        self.assertEqual(10584, len(reduced))

    def test_with_ignore(self):
        polymer = "dabAcCaCBAcCcaDA"
        self.assertEqual("dbCBcD", polymer_reduce(polymer, ignores='aA'))
        self.assertEqual("dbCBcD", polymer_reduce(polymer, ignores='Aa'))
        self.assertEqual("daCAcaDA", polymer_reduce(polymer, ignores='bB'))
        self.assertEqual("daCAcaDA", polymer_reduce(polymer, ignores='Bb'))
        self.assertEqual("daDA", polymer_reduce(polymer, ignores='cC'))
        self.assertEqual("daDA", polymer_reduce(polymer, ignores='Cc'))
        self.assertEqual("abCBAc", polymer_reduce(polymer, ignores='dD'))
        self.assertEqual("abCBAc", polymer_reduce(polymer, ignores='Dd'))

    def test_find_shortest_with_ignores(self):
        self.assertEqual("daDA", find_shortest_with_ignores("dabAcCaCBAcCcaDA"))

    @unittest.skip("Too slow")
    def test_find_shortest_with_ignores_mine(self):
        polymer = data_text(2018, "day_05_mine.txt")
        best_reduced = find_shortest_with_ignores(polymer)
        self.assertEqual(6968, len(best_reduced))


def is_opposite_case(letter1, letter2):
    return abs(ord(letter1) - ord(letter2)) == ord("a") - ord("A")


def find_shortest_with_ignores(polymer):
    shortest = None
    for ignore_index in range(ord('Z')-ord('A') + 1):
        ignore_min = chr(ord('a') + ignore_index)
        ignore_maj = chr(ord('A') + ignore_index)
        reduced = polymer_reduce(polymer, ignores=ignore_min+ignore_maj)
        if shortest is None or len(reduced) < len(shortest):
            shortest = reduced

    return shortest


def polymer_reduce(polymer, ignores=""):
    poly_list = [char for char in polymer]
    current = 0
    while current < len(poly_list):
        if poly_list[current] in ignores:
            del poly_list[current]
            current = max(0, current - 1)
        elif current >= len(poly_list) - 1:
            break
        elif is_opposite_case(poly_list[current], poly_list[current + 1]):
            del poly_list[current:current+2]
            current = max(0, current - 1)
        else:
            current += 1
    return "".join(poly_list)
