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

    def test_execute_example(self):
        processor = IntCodeProcessor([1, 1, 1, 4, 99, 5, 6, 0, 99], instructions_day_02)
        processor.execute()
        self.assertListEqual([30, 1, 1, 4, 2, 5, 6, 0, 99], processor.memory)


class EndProgram(Exception):
    pass


class Instruction:
    def __init__(self, operation, num_arguments):
        self.operation = operation
        self.num_arguments = num_arguments

    def __call__(self, address, memory):
        arguments = memory[address+1:address+1+self.num_arguments]
        a_index = arguments[0]
        b_index = arguments[1]
        c_index = arguments[2]
        memory[c_index] = self.operation(memory[a_index], memory[b_index])

    @property
    def size(self):
        return self.num_arguments + 1


class NoopInstruction(Instruction):
    def __init__(self):
        super(NoopInstruction, self).__init__(None, 0)

    def __call__(self, *args, **kwargs):
        pass


class EndProgramInstruction(Instruction):
    def __init__(self):
        super(EndProgramInstruction, self).__init__(None, 0)

    def __call__(self, *args, **kwargs):
        raise EndProgram


class IntCodeProcessor:
    def __init__(self, initial_memory, instruction_set):
        self.memory = initial_memory
        self.instruction_pointer = 0
        self.instructions = instruction_set

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
        operation_code = self.memory[address]
        instruction = self.instructions[operation_code]
        instruction(address, self.memory)
        return instruction.size


instructions_day_02 = {
    0: NoopInstruction(),
    1: Instruction(operator.add, 3),
    2: Instruction(operator.mul, 3),
    99: EndProgramInstruction(),
}
