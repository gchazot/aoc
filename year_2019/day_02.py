import itertools
import unittest

from aoc_utils.data import data_text
from year_2019.int_code_processor import IntCodeProcessor, instructions_day_02


class TestIntCodeProcessor(unittest.TestCase):
    def test_initialise(self):
        processor = IntCodeProcessor([1, 2, 3, 4, 0], instructions_day_02)
        self.assertListEqual([1, 2, 3, 4, 0], processor.memory)
        self.assertEqual(instructions_day_02, processor.instructions)

    def test_execute_example_instructions(self):
        def check_instruction(expected_memory, address, initial_state):
            processor = IntCodeProcessor(initial_state, instructions_day_02)
            processor.execute_instruction_at(address)
            self.assertListEqual(expected_memory, processor.memory)

        check_instruction([2, 0, 0, 0, 99], 0, [1, 0, 0, 0, 99])
        check_instruction([2, 3, 0, 6, 99], 0, [2, 3, 0, 3, 99])
        check_instruction([2, 4, 4, 5, 99, 9801], 0, [2, 4, 4, 5, 99, 0])

        check_instruction([1, 1, 1, 4, 2, 5, 6, 0, 99], 0, [1, 1, 1, 4, 99, 5, 6, 0, 99])
        check_instruction([30, 1, 1, 4, 2, 5, 6, 0, 99], 4, [1, 1, 1, 4, 2, 5, 6, 0, 99])

    def test_execute_example(self):
        processor = IntCodeProcessor([1, 1, 1, 4, 99, 5, 6, 0, 99], instructions_day_02)
        processor.execute()
        self.assertListEqual([30, 1, 1, 4, 2, 5, 6, 0, 99], processor.memory)

    def test_execute_mine(self):
        initial_memory = list(map(int, data_text(2019, "day_02_mine.txt").split(",")))
        initial_memory[1] = 12
        initial_memory[2] = 2

        processor = IntCodeProcessor(initial_memory, instructions_day_02)
        try:
            processor.execute()
        except RuntimeError:
            pass
        self.assertEqual(3058646, processor.output)

    def test_find_noun_and_verb(self):
        base_memory = list(map(int, data_text(2019, "day_02_mine.txt").split(",")))
        for noun, verb in itertools.product(range(100), range(100)):
            initial_memory = base_memory.copy()
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


