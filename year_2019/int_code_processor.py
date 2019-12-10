import functools
import unittest
import operator


class TestIntCodeProcessor(unittest.TestCase):
    def test_initialise(self):
        fake_instructions = {42: 'blah'}
        processor = IntCodeProcessor([1, 2, 3, 4, 0], fake_instructions)
        self.assertListEqual([1, 2, 3, 4, 0], processor.memory)
        self.assertEqual(fake_instructions, processor.instructions)

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

    def test_execute_mode_0(self):
        day_2_assert = functools.partial(self._assert_result, instructions=instructions_day_02)
        day_2_assert([30, 1, 1, 4, 2, 5, 6, 0, 99], [1, 1, 1, 4, 99, 5, 6, 0, 99])

    def test_execute_mode_1(self):
        day_2_assert = functools.partial(self._assert_result, instructions=instructions_day_02)

        day_2_assert([1002, 4, 3, 4, 99], [1002, 4, 3, 4, 33])
        day_2_assert([1101, 100, -1, 4, 99], [1101, 100, -1, 4, 0])

    def test_execute_input_instruction(self):
        day_5_assert = functools.partial(self._assert_result, instructions=instructions_day_05_1)

        fake_input = 42
        day_5_assert([fake_input, 0, 4, 0, 99], [3, 0, 4, 0, 99],
                     input_values=[fake_input], expected_output=[fake_input])

    def _assert_result(
            self, expected_memory, initial_memory, instructions,
            input_values=None, expected_output=None):
        processor = IntCodeProcessor(initial_memory, instructions, input_values)
        processor.execute()
        self.assertListEqual(expected_memory, processor.memory)
        if expected_output is not None:
            self.assertListEqual(expected_output, processor.output_values)


class Instruction:
    def size(self):
        return 1

    def __call__(self, *args, **kwargs):
        raise NotImplementedError


class NoopInstruction(Instruction):
    def __call__(self, *args, **kwargs):
        pass


class EndProgramInstruction(Instruction):
    def __call__(self, *args, **kwargs):
        raise EndProgram


class FunctionInstruction(Instruction):
    def __init__(self, operation, num_arguments):
        self.operation = operation
        self.num_arguments = num_arguments

    def size(self):
        return self.num_arguments + 1

    def __call__(self, address, memory, **kwargs):
        operation_code = memory[address]
        modes = operation_code // 100
        arguments = []
        for i in range(self.num_arguments):
            mode = modes % 10
            modes //= 10
            arguments.append(ArgumentWrapper(memory, address + 1 + i, mode))

        result = self.operation(arguments[0].get(), arguments[1].get())
        arguments[2].set(result)


class InputInstruction(Instruction):
    def size(self):
        return 2

    def __call__(self, address, memory, input_values, **kwargs):
        operation_code = memory[address]
        mode = operation_code // 100
        argument = ArgumentWrapper(memory, address+1, mode)
        input_value = input_values.pop(0)
        argument.set(input_value)


class OutputInstruction(Instruction):
    def size(self):
        return 2

    def __call__(self, address, memory, **kwargs):
        operation_code = memory[address]
        mode = operation_code // 100
        argument = ArgumentWrapper(memory, address+1, mode)
        return argument.get()



class ArgumentWrapper:
    def __init__(self, memory, address, mode):
        self.memory = memory
        self.address = address
        self.mode = mode

    def get(self):
        if self.mode == 0:
            return self.memory[self.memory[self.address]]
        elif self.mode == 1:
            return self.memory[self.address]
        raise RuntimeError("Unknown mode {0}".format(self.mode))

    def set(self, value):
        if self.mode == 0:
            self.memory[self.memory[self.address]] = value
        elif self.mode == 1:
            self.memory[self.address] = value
        else:
            raise RuntimeError("Unknown mode {0}".format(self.mode))


instructions_day_02 = {
    0: NoopInstruction(),
    1: FunctionInstruction(operator.add, 3),
    2: FunctionInstruction(operator.mul, 3),
    99: EndProgramInstruction(),
}

instructions_day_05_1 = {
    **instructions_day_02,
    3: InputInstruction(),
    4: OutputInstruction(),
}


class IntCodeProcessor:
    def __init__(self, initial_memory, instruction_set, input_values=None):
        self.memory = initial_memory
        self.instruction_pointer = 0
        self.instructions = instruction_set
        self.input_values = input_values or []
        self.output_values = []

    @property
    def output(self):
        return self.memory[0]

    def execute(self):
        while self.instruction_pointer < len(self.memory):
            try:
                instruction_size = self.execute_instruction_at(self.instruction_pointer)
            except EndProgram:
                return
            self.instruction_pointer += instruction_size

    def execute_instruction_at(self, address):
        operation_code = self.memory[address] % 100
        instruction = self.instructions[operation_code]
        result = instruction(address=address, memory=self.memory, input_values=self.input_values)
        if result is not None:
            self.output_values.append(result)
        return instruction.size()


class EndProgram(Exception):
    pass
