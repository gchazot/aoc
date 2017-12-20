import unittest


class Generator:
    def __init__(self, start, factor):
        self.value = start
        self.factor = factor

    DIVIDER = 2147483647
    MASK = (1 << 16) - 1

    def step(self):
        self.value = (self.value * self.factor) % self.DIVIDER

    @property
    def hash(self):
        return self.value & self.MASK


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


class TestGenerator(unittest.TestCase):
    def use_example_gens(self):
        self.genA = Generator(65, 16807)
        self.genB = Generator(8921, 48271)

    def use_my_generators(self):
        self.genA = Generator(116, 16807)
        self.genB = Generator(299, 48271)

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
