import unittest

from aoc_utils.data import data_text
from year_2019.int_code_processor import (
    IntCodeProcessor, instructions_day_05_1, instructions_day_05_2,
)


class TestIntCodeProcessor(unittest.TestCase):
    def test_execute_part_1_mine(self):
        initial_memory = list(map(int, data_text(2019, "day_05_mine.txt").split(",")))
        input_values = [1]

        processor = IntCodeProcessor(
            initial_memory,
            instructions_day_05_1,
            input_values=input_values,
        )
        processor.execute()

        self.assertTrue(all(value == 0 for value in processor.output_values[:-1]))
        self.assertEqual(7157989, processor.output_values[-1])

    def test_execute_part_2_example(self):
        example = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
        ]

        for value in range(20):
            processor = IntCodeProcessor(
                example,
                instructions_day_05_2,
                input_values=[value],
            )
            processor.execute()
            expected_output = 999 if value < 8 else 1000 if value == 8 else 1001
            self.assertListEqual([expected_output], processor.output_values)

    def test_execute_part_2_mine(self):
        initial_memory = list(map(int, data_text(2019, "day_05_mine.txt").split(",")))
        input_values = [5]

        processor = IntCodeProcessor(
            initial_memory,
            instructions_day_05_2,
            input_values=input_values,
        )
        processor.execute()

        self.assertListEqual([7873292], processor.output_values)