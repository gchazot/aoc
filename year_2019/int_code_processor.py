from collections import defaultdict
import functools
import unittest
import operator


class TestIntCodeProcessor(unittest.TestCase):
    def test_initialise(self):
        fake_instructions = {42: 'blah'}
        processor = IntCodeProcessor([1, 2, 3, 4, 0], fake_instructions)
        self.assertEqual([1, 2, 3, 4, 0], processor.memory)
        self.assertEqual(fake_instructions, processor.instructions)

    def test_execute_example_instructions(self):
        def check_instruction(expected_memory, address, initial_state):
            processor = IntCodeProcessor(initial_state, instructions_day_02)
            processor.execute_instruction_at(address)
            self.assertEqual(expected_memory, processor.memory)

        check_instruction([2, 0, 0, 0, 99], 0, [1, 0, 0, 0, 99])
        check_instruction([2, 3, 0, 6, 99], 0, [2, 3, 0, 3, 99])
        check_instruction([2, 4, 4, 5, 99, 9801], 0, [2, 4, 4, 5, 99, 0])

        check_instruction([1, 1, 1, 4, 2, 5, 6, 0, 99], 0, [1, 1, 1, 4, 99, 5, 6, 0, 99])
        check_instruction([30, 1, 1, 4, 2, 5, 6, 0, 99], 4, [1, 1, 1, 4, 2, 5, 6, 0, 99])

    def test_execute_mode_0(self):
        day_2_assert = functools.partial(self._assert_result, instructions=instructions_day_02)
        day_2_assert(
            expected_memory=[30, 1, 1, 4, 2, 5, 6, 0, 99],
            initial_memory=[1, 1, 1, 4, 99, 5, 6, 0, 99],
        )

    def test_execute_mode_1(self):
        day_2_assert = functools.partial(self._assert_result, instructions=instructions_day_02)

        day_2_assert(expected_memory=[1002, 4, 3, 4, 99], initial_memory=[1002, 4, 3, 4, 33])
        day_2_assert(expected_memory=[1101, 100, -1, 4, 99], initial_memory=[1101, 100, -1, 4, 0])

    def test_execute_mode_2(self):
        day_9_assert = functools.partial(self._assert_result, instructions=instructions_day_09)

        day_9_assert(
            expected_output=[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            initial_memory=[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        )
        day_9_assert(
            expected_output=[1219070632396864],
            initial_memory=[1102, 34915192, 34915192, 7, 4, 7, 99, 0],
        )
        day_9_assert(
            expected_output=[1125899906842624],
            initial_memory=[104, 1125899906842624, 99],
        )

    def test_execute_input_instruction(self):
        day_5_assert = functools.partial(self._assert_result, instructions=instructions_day_05_1)

        fake_input = 42
        day_5_assert(expected_memory=[fake_input, 0, 4, 0, 99], initial_memory=[3, 0, 4, 0, 99],
                     input_values=[fake_input], expected_output=[fake_input])

    def test_execute_compare(self):
        day_5_2_assert = functools.partial(
            self._assert_result,
            instructions=instructions_day_05_2,
            expected_memory=None,
        )

        for input_value in (7, 8, 9):
            day_5_2_assert(
                [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
                input_values=[input_value],
                expected_output=[1 if input_value == 8 else 0],
            )
            day_5_2_assert(
                [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
                input_values=[input_value],
                expected_output=[1 if input_value < 8 else 0],
            )
            day_5_2_assert(
                [3, 3, 1108, -1, 8, 3, 4, 3, 99],
                input_values=[input_value],
                expected_output=[1 if input_value == 8 else 0],
            )
            day_5_2_assert(
                [3, 3, 1107, -1, 8, 3, 4, 3, 99],
                input_values=[input_value],
                expected_output=[1 if input_value < 8 else 0],
            )

    def test_execute_jump(self):
        day_5_2_assert = functools.partial(
            self._assert_result,
            instructions=instructions_day_05_2,
            expected_memory=None,
        )

        for input_value in (-1, 0, 1):
            day_5_2_assert(
                [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                input_values=[input_value],
                expected_output=[0 if input_value == 0 else 1],
            )
            day_5_2_assert(
                [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                input_values=[input_value],
                expected_output=[0 if input_value == 0 else 1],
            )

    def _assert_result(
            self, initial_memory, instructions, expected_memory=None,
            input_values=None, expected_output=None
    ):
        processor = IntCodeProcessor(initial_memory, instructions, input_values)
        processor.execute()
        if expected_memory is not None:
            self.assertEqual(expected_memory, processor.memory)
        if expected_output is not None:
            self.assertEqual(expected_output, processor.output_values)


class Instruction:
    def size(self):
        return 1

    def __call__(self, *args, **kwargs):
        raise NotImplementedError

    @staticmethod
    def arguments(address, memory, num_arguments):
        operation_code = memory[address]
        modes = operation_code // 100
        arguments = []
        for i in range(num_arguments):
            mode = modes % 10
            modes //= 10
            arguments.append(ArgumentWrapper(memory, address + 1 + i, mode))
        return arguments


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
        arguments = self.arguments(address, memory, self.num_arguments)
        result = self.operation(arguments[0].get(), arguments[1].get())
        arguments[2].set(result)


class InputInstruction(Instruction):
    def size(self):
        return 2

    def __call__(self, address, memory, input_values, **kwargs):
        argument = self.arguments(address, memory, 1)[0]
        try:
            input_value = input_values.pop(0)
        except IndexError:
            raise InputNeeded()
        argument.set(input_value)


class OutputInstruction(Instruction):
    def size(self):
        return 2

    def __call__(self, address, memory, **kwargs):
        argument = self.arguments(address, memory, 1)[0]
        return argument.get()


class JumpIfTrueInstruction(Instruction):
    def size(self):
        return 3

    def __call__(self, address, memory, **kwargs):
        arguments = self.arguments(address, memory, 2)
        if arguments[0].get() != 0:
            raise Jump(arguments[1].get())


class JumpIfFalseInstruction(Instruction):
    def size(self):
        return 3

    def __call__(self, address, memory, **kwargs):
        arguments = self.arguments(address, memory, 2)
        if arguments[0].get() == 0:
            raise Jump(arguments[1].get())


class LessThanInstructions(Instruction):
    def size(self):
        return 4

    def __call__(self, address, memory, **kwargs):
        arguments = self.arguments(address, memory, 3)
        if arguments[0].get() < arguments[1].get():
            arguments[2].set(1)
        else:
            arguments[2].set(0)


class EqualsInstruction(Instruction):
    def size(self):
        return 4

    def __call__(self, address, memory, **kwargs):
        arguments = self.arguments(address, memory, 3)
        if arguments[0].get() == arguments[1].get():
            arguments[2].set(1)
        else:
            arguments[2].set(0)


class AdjustRelativeBaseInstruction(Instruction):
    def size(self):
        return 2

    def __call__(self, address, memory, **kwargs):
        argument = self.arguments(address, memory, 1)[0]
        memory.relative_base += argument.get()


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
        elif self.mode == 2:
            return self.memory[self.memory.relative_base + self.memory[self.address]]
        raise RuntimeError("Unknown mode {0}".format(self.mode))

    def set(self, value):
        if self.mode == 0:
            self.memory[self.memory[self.address]] = value
        elif self.mode == 1:
            self.memory[self.address] = value
        elif self.mode == 2:
            self.memory[self.memory.relative_base + self.memory[self.address]] = value
        else:
            raise RuntimeError("Unknown mode {0}".format(self.mode))


instructions_day_02 = {
    0: NoopInstruction(),
    1: FunctionInstruction(operator.add, 3),
    2: FunctionInstruction(operator.mul, 3),
    99: EndProgramInstruction(),
}

instructions_day_05_1 = instructions_day_02.copy()
instructions_day_05_1.update({
    3: InputInstruction(),
    4: OutputInstruction(),
})

instructions_day_05_2 = instructions_day_05_1.copy()
instructions_day_05_2.update({
    5: JumpIfTrueInstruction(),
    6: JumpIfFalseInstruction(),
    7: LessThanInstructions(),
    8: EqualsInstruction(),
})

instructions_day_09 = instructions_day_05_2.copy()
instructions_day_09.update({
    9: AdjustRelativeBaseInstruction(),
})


class IntCodeProcessor:
    def __init__(self, initial_memory, instruction_set, input_values=None, output_values=None):
        self.memory = InfiniteMemory(initial_memory)
        self.instruction_pointer = 0
        self.instructions = instruction_set
        self.input_values = input_values if input_values is not None else []
        self.output_values = output_values if output_values is not None else []

    @property
    def output(self):
        return self.memory[0]

    def execute(self):
        while self.instruction_pointer < len(self.memory):
            try:
                instruction_size = self.execute_instruction_at(self.instruction_pointer)
            except EndProgram:
                return
            except Jump as e:
                self.instruction_pointer = e.to_address
            else:
                self.instruction_pointer += instruction_size

    def execute_instruction_at(self, address):
        operation_code = self.memory[address] % 100
        instruction = self.instructions[operation_code]
        result = instruction(address=address, memory=self.memory, input_values=self.input_values)
        if result is not None:
            self.output_values.append(result)
        return instruction.size()


class InfiniteMemory(defaultdict):
    def __init__(self, initial_memory):
        self.relative_base = 0
        super(InfiniteMemory, self).__init__(int, enumerate(initial_memory))

    def __getitem__(self, index):
        if index < 0:
            raise IndexError
        return super(InfiniteMemory, self).__getitem__(index)

    def __setitem__(self, index, value):
        if index < 0:
            raise IndexError
        super(InfiniteMemory, self).__setitem__(index, value)

    def __eq__(self, other):
        if isinstance(other, (tuple, list)):
            return (
                    len(self) == len(other) and
                    min(self.keys()) == 0 and
                    max(self.keys()) == len(self) - 1 and
                    all(self[i] == other[i] for i in range(len(other)))
            )
        else:
            return super(InfiniteMemory, self).__eq__(other)


class EndProgram(Exception):
    pass


class InputNeeded(Exception):
    pass


class Jump(Exception):
    def __init__(self, to_address):
        self.to_address = to_address
