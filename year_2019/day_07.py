from itertools import permutations
import unittest
from year_2019.int_code_processor import (
    IntCodeProcessor,
    instructions_day_05_2,
    InputNeeded,
    read_program,
)


class TestSimpleAmplifier(unittest.TestCase):
    def test_amplifier_example_1(self):
        program = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
        phases = (4, 3, 2, 1, 0)

        value = calculate_output_with_feedback(phases, program)

        self.assertEqual(43210, value)

    def test_amplifier_example_2(self):
        program = [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                   101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]
        phases = (0, 1, 2, 3, 4)

        value = calculate_output_with_feedback(phases, program)

        self.assertEqual(54321, value)

    def test_amplifier_example_3(self):
        program = [3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                   1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]
        phases = (1, 0, 4, 3, 2)

        value = calculate_output_with_feedback(phases, program)

        self.assertEqual(65210, value)

    def test_optimiser_example_1(self):
        program = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]

        phases, value = optimise_output(program, [0, 1, 2, 3, 4])

        self.assertEqual((4, 3, 2, 1, 0), phases)
        self.assertEqual(43210, value)

    def test_optimiser_example_2(self):
        program = [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                   101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]

        phases, value = optimise_output(program, [0, 1, 2, 3, 4])

        self.assertEqual((0, 1, 2, 3, 4), phases)
        self.assertEqual(54321, value)

    def test_optimiser_example_3(self):
        program = [3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                   1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]

        phases, value = optimise_output(program, [0, 1, 2, 3, 4])

        self.assertEqual((1, 0, 4, 3, 2), phases)
        self.assertEqual(65210, value)

    def test_optimiser_mine(self):
        program = read_program("day_07_mine.txt")

        phases, value = optimise_output(program, [0, 1, 2, 3, 4])

        self.assertEqual((2, 1, 4, 0, 3), phases)
        self.assertEqual(262086, value)


class TestLoopbackAmplifier(unittest.TestCase):
    def test_amplifier_example_1(self):
        program = [3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                   27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5]
        phases = (9, 8, 7, 6, 5)

        value = calculate_output_with_feedback(phases, program)

        self.assertEqual(139629729, value)

    def test_amplifier_example_2(self):
        program = [3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
                   -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
                   53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10]
        phases = (9, 7, 8, 5, 6)

        value = calculate_output_with_feedback(phases, program)

        self.assertEqual(18216, value)


    def test_optimiser_example_1(self):
        program = [3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                   27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5]

        phases, value = optimise_output(program, [5, 6, 7, 8, 9])

        self.assertEqual((9, 8, 7, 6, 5), phases)
        self.assertEqual(139629729, value)

    def test_optimiser_example_2(self):
        program = [3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
                   -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
                   53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10]

        phases, value = optimise_output(program, [5, 6, 7, 8, 9])

        self.assertEqual((9, 7, 8, 5, 6), phases)
        self.assertEqual(18216, value)

    def test_optimiser_mine(self):
        program = read_program("day_07_mine.txt")

        phases, value = optimise_output(program, [5, 6, 7, 8, 9])

        self.assertEqual((5, 7, 6, 8, 9), phases)
        self.assertEqual(5371621, value)


def optimise_output(program, valid_phases):
    results = [
        (phases, calculate_output_with_feedback(phases, program))
        for phases in permutations(valid_phases)
    ]
    return max(results, key=lambda pair: pair[1])


def calculate_output_with_feedback(phases, program):
    processors = [
        IntCodeProcessor(
            program[:],
            instructions_day_05_2,
            input_values=None,
        )
        for _ in phases
    ]

    for i, phase in enumerate(phases):
        processors[i].input_values = processors[i-1].output_values
        processors[i].input_values.append(phase)

    processors[0].input_values.append(0)

    running = [True for _ in processors]
    while any(running):
        for i in range(len(processors)):
            if running[i]:
                try:
                    processors[i].execute()
                except InputNeeded:
                    continue
                else:
                    running[i] = False

    assert 1 == len(processors[-1].output_values)
    value = processors[-1].output_values[0]
    return value
