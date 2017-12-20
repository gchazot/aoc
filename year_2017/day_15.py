import unittest


class Generator:
    def __init__(self, start, factor, criterion):
        self.value = start
        self.factor = factor
        self.criterion = criterion

    DIVIDER = 2147483647
    MASK = (1 << 16) - 1

    def step(self):
        self.value = (self.value * self.factor) % self.DIVIDER

    @property
    def hash(self):
        return self.value & self.MASK

    def gen_values(self, num_values):
        i = 0
        while i < num_values:
            self.step()
            if self.value % self.criterion == 0:
                yield self.value
                i += 1


def compare_generators(gen_a, gen_b):
    return gen_a.hash == gen_b.hash


def count_matches(gen_a, gen_b, iterations):
    count = 0
    for _ in range(iterations):
        gen_a.step()
        gen_b.step()
        if compare_generators(gen_a, gen_b):
            count += 1
    return count


def count_picky_matches(gen_a, gen_b, iterations):
    values = zip(gen_a.gen_values(iterations), gen_b.gen_values(iterations))
    return sum((1 for pair in values if (pair[0] - pair[1]) & Generator.MASK == 0))


class TestGenerator(unittest.TestCase):
    def use_example_gens(self):
        self.genA = Generator(65, 16807, 4)
        self.genB = Generator(8921, 48271, 8)

    def use_my_generators(self):
        self.genA = Generator(116, 16807, 4)
        self.genB = Generator(299, 48271, 8)

    def step_generators(self):
        self.genA.step()
        self.genB.step()

    def test_initial_value_is_start(self):
        self.use_example_gens()
        self.assertEqual(65, self.genA.value)
        self.assertEqual(8921, self.genB.value)

    def test_gen_next_value_examples(self):
        self.use_example_gens()

        self.step_generators()
        self.assertEqual(1092455, self.genA.value)
        self.assertEqual(430625591, self.genB.value)

        self.step_generators()
        self.assertEqual(1181022009, self.genA.value)
        self.assertEqual(1233683848, self.genB.value)

        self.step_generators()
        self.assertEqual(245556042, self.genA.value)
        self.assertEqual(1431495498, self.genB.value)

        self.step_generators()
        self.assertEqual(1744312007, self.genA.value)
        self.assertEqual(137874439, self.genB.value)

        self.step_generators()
        self.assertEqual(1352636452, self.genA.value)
        self.assertEqual(285222916, self.genB.value)

    def test_comparison(self):
        self.use_example_gens()

        self.step_generators()
        self.assertFalse(compare_generators(self.genA, self.genB))

        self.step_generators()
        self.assertFalse(compare_generators(self.genA, self.genB))

        self.step_generators()
        self.assertTrue(compare_generators(self.genA, self.genB))

        self.step_generators()
        self.assertFalse(compare_generators(self.genA, self.genB))

        self.step_generators()
        self.assertFalse(compare_generators(self.genA, self.genB))

    def test_count_comparisons_example(self):
        self.use_example_gens()
        self.assertEqual(1, count_matches(self.genA, self.genB, 5))

    @unittest.skip("Taking too long for now")
    def test_count_comparisons_example_long(self):
        self.use_example_gens()
        self.assertEqual(588, count_matches(self.genA, self.genB, 40000000))

    @unittest.skip("Taking too long for now")
    def test_count_comparisons_mine(self):
        self.use_my_generators()
        self.assertEqual(569, count_matches(self.genA, self.genB, 40000000))

    def test_gen_next_picky_value_examples(self):
        self.use_example_gens()

        values = zip(self.genA.gen_values(5), self.genB.gen_values(5))
        self.assertListEqual([
            (1352636452, 1233683848),
            (1992081072, 862516352),
            (530830436, 1159784568),
            (1980017072, 1616057672),
            (740335192, 412269392),
        ], list(values))

    @unittest.skip("Taking too long for now")
    def test_count_picky_comparisons_example(self):
        self.use_example_gens()

        self.assertEqual(309, count_picky_matches(self.genA, self.genB, 5000000))

    @unittest.skip("Taking too long for now")
    def test_count_picky_comparisons_mine(self):
        self.use_my_generators()

        self.assertEqual(298, count_picky_matches(self.genA, self.genB, 5000000))
