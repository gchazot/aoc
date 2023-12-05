import unittest
from array import array

from aoc_utils import test


class TestFactory(unittest.TestCase):
    def _check_array(self, factory, recipes, elves):
        self.assertListEqual(recipes, list(factory.recipes))
        for i, elf in enumerate(elves):
            self.assertEqual(elf, factory.elf[i])

    def test_initial_condition(self):
        factory = Factory()
        self._check_array(factory, [3, 7], [0, 1])

    def test_iterate(self):
        factory = Factory()

        factory.iterate()
        self._check_array(factory, [3, 7, 1, 0], [0, 1])

        factory.iterate()
        self._check_array(factory, [3, 7, 1, 0, 1, 0], [4, 3])

        factory.iterate()
        self._check_array(factory, [3, 7, 1, 0, 1, 0, 1], [6, 4])

    def test_scores_after(self):
        factory = Factory()
        self.assertEqual("5158916779", factory.score_after(9))
        self.assertEqual("0124515891", factory.score_after(5))
        self.assertEqual("9251071085", factory.score_after(18))
        self.assertEqual("5941429882", factory.score_after(2018))

        self.assertEqual("3147574107", factory.score_after(293801))

    def test_first_occurence_of(self):
        factory = Factory()
        self.assertEqual(9, factory.first_occurence_of("51589"))
        self.assertEqual(5, factory.first_occurence_of("01245"))
        self.assertEqual(18, factory.first_occurence_of("92510"))
        self.assertEqual(2018, factory.first_occurence_of("59414"))

    @test.pypy_only("Taking too long")
    def test_first_occurence_of_mine(self):
        factory = Factory()
        self.assertEqual(20280190, factory.first_occurence_of("293801"))


class Factory:
    def __init__(self):
        self.recipes = array('B', [3, 7])
        self.elf = list(range(2))

    def first_occurence_of(self, wanted_scores):
        wanted_scores = list(map(int, wanted_scores))
        current = 0
        offset = 0
        while offset < len(wanted_scores):
            if current + len(wanted_scores) > len(self.recipes):
                self.iterate()
            elif self.recipes[current + offset] == wanted_scores[offset]:
                offset += 1
            else:
                offset = 0
                current += 1

        return current

    def score_after(self, num_recipes):
        score_length = 10

        while len(self.recipes) < num_recipes + score_length:
            self.iterate()

        next_scores = self.recipes[num_recipes:num_recipes + score_length]
        return "".join(map(str, next_scores))

    def iterate(self):
        total = sum(self.recipes[recipe] for recipe in self.elf)
        digits = map(int, str(total))
        self.recipes.extend(digits)

        for elf, recipe in enumerate(self.elf):
            next_recipe = recipe + 1 + self.recipes[recipe]
            next_recipe %= len(self.recipes)
            self.elf[elf] = next_recipe
