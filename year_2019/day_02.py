import copy
import itertools
import unittest

from year_2019.int_code_processor import IntCodeProcessor, instructions_day_02, read_program


class TestIntCodeProcessor(unittest.TestCase):
    def test_execute_mine(self):
        initial_memory = read_program("day_02_mine.txt")
        initial_memory[1] = 12
        initial_memory[2] = 2

        processor = IntCodeProcessor(initial_memory, instructions_day_02)
        try:
            processor.execute()
        except RuntimeError:
            pass
        self.assertEqual(3058646, processor.output)

    def test_find_noun_and_verb(self):
        base_memory = read_program("day_02_mine.txt")
        for noun, verb in itertools.product(range(100), range(100)):
            initial_memory = copy.copy(base_memory)
            initial_memory[1] = noun
            initial_memory[2] = verb

            processor = IntCodeProcessor(initial_memory, instructions_day_02)
            try:
                processor.execute()
            except RuntimeError:
                pass

            if processor.output == 19690720:
                break

        self.assertEqual(89, noun)
        self.assertEqual(76, verb)
        self.assertEqual(8976, 100 * noun + verb)


