import unittest

from aoc_utils.data import data_text
from year_2019.int_code_processor import IntCodeProcessor, instructions_day_05


class TestIntCodeProcessor(unittest.TestCase):
    def test_execute_mine(self):
        initial_memory = list(map(int, data_text(2019, "day_05_mine.txt").split(",")))
        input_values = [1]

        processor = IntCodeProcessor(initial_memory, instructions_day_05, input_values=input_values)
        processor.execute()

        self.assertTrue(all(value==0 for value in processor.output_values[:-1]))
        self.assertEqual(7157989, processor.output_values[-1])

