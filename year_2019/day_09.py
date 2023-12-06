import unittest

from year_2019.int_code_processor import (
    IntCodeProcessor,
    instructions_day_09,
    read_program,
)


class TestBoostProgram(unittest.TestCase):
    def test_boost_test_mine(self):
        program = read_program("day_09_mine.txt")

        processor = IntCodeProcessor(program, instructions_day_09, input_values=[1])
        processor.execute()

        self.assertEqual([3241900951], processor.output_values)

    def test_boost_run_mine(self):
        program = read_program("day_09_mine.txt")

        processor = IntCodeProcessor(program, instructions_day_09, input_values=[2])
        processor.execute()

        self.assertEqual([83089], processor.output_values)
