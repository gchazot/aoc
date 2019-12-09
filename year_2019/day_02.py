import inspect
import itertools
import operator
import unittest

from aoc_utils.data import data_text


class TestIntCodeProcessor(unittest.TestCase):
    def test_initialise(self):
        processor = IntCodeProcessor([1, 2, 3, 4, 0], initial_instructions)
        self.assertListEqual([1, 2, 3, 4, 0], processor.memory)
        self.assertEqual(initial_instructions, processor.instructions)

    def test_execute_example_instructions(self):
        def check_instruction(expected_memory, address, initial_state):
            processor = IntCodeProcessor(initial_state, initial_instructions)
            processor.execute_instruction_at(address)
            self.assertListEqual(expected_memory, processor.memory)

        check_instruction([2, 0, 0, 0, 99], 0, [1, 0, 0, 0, 99])
        check_instruction([2, 3, 0, 6, 99], 0, [2, 3, 0, 3, 99])
        check_instruction([2, 4, 4, 5, 99, 9801], 0, [2, 4, 4, 5, 99, 0])

        check_instruction([1, 1, 1, 4, 2, 5, 6, 0, 99], 0, [1, 1, 1, 4, 99, 5, 6, 0, 99])
        check_instruction([30, 1, 1, 4, 2, 5, 6, 0, 99], 4, [1, 1, 1, 4, 2, 5, 6, 0, 99])

    def test_execute_example(self):
        processor = IntCodeProcessor([1, 1, 1, 4, 99, 5, 6, 0, 99], initial_instructions)
        processor.execute()
        self.assertListEqual([30, 1, 1, 4, 2, 5, 6, 0, 99], processor.memory)

    def test_execute_mine(self):
        initial_memory = list(map(int, data_text(2019, "day_02_mine.txt").split(",")))
        initial_memory[1] = 12
        initial_memory[2] = 2

        processor = IntCodeProcessor(initial_memory, initial_instructions)
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

            processor = IntCodeProcessor(initial_memory, initial_instructions)
            try:
                processor.execute()
            except RuntimeError:
                pass

            if processor.output == 19690720:
                break

        self.assertEqual(89, noun)
        self.assertEqual(76, verb)
        self.assertEqual(8976, 100 * noun + verb)


class EndProgram(Exception):
    pass


initial_instructions = {
    0: None,
    1: operator.add,
    2: operator.mul,
    99: EndProgram,
}


class IntCodeProcessor:
    def __init__(self, initial_memory, instruction_set):
        self.memory = initial_memory
        self.instruction_pointer = 0
        self.instruction_size = 4
        self.instructions = instruction_set

    @property
    def output(self):
        return self.memory[0]

    def execute(self):
        while self.instruction_pointer < len(self.memory):
            self.execute_instruction_at(self.instruction_pointer)
            self.instruction_pointer += self.instruction_size

    def execute_instruction_at(self, address):
        instruction = self.get_instruction(address)
        self.execute_instruction(instruction)

    def get_instruction(self, address):
        return self.memory[address:address + 4]

    def execute_instruction(self, instruction):
        operation_code = instruction[0]
        operation = self.instructions[operation_code]

        if operation == EndProgram:
            return
        elif operation is None:
            return
        elif inspect.isbuiltin(operation) or inspect.isfunction(operation):
            a_index = instruction[1]
            b_index = instruction[2]
            c_index = instruction[3]
            self.memory[c_index] = operation(self.memory[a_index], self.memory[b_index])
            return
        else:
            raise RuntimeError("Unknown operation_code {0}".format(operation_code))
