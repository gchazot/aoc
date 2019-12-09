import inspect
import operator


class EndProgram(Exception):
    pass


class Instruction:
    def __init__(self, operation, num_arguments):
        self.operation = operation
        self.num_arguments = num_arguments

    def __call__(self, address, memory):
        if self.operation == EndProgram:
            raise EndProgram
        elif self.operation is None:
            pass
        elif inspect.isbuiltin(self.operation) or inspect.isfunction(self.operation):
            arguments = memory[address+1:address+1+self.num_arguments]
            a_index = arguments[0]
            b_index = arguments[1]
            c_index = arguments[2]
            memory[c_index] = self.operation(memory[a_index], memory[b_index])

    @property
    def size(self):
        return self.num_arguments + 1


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
    0: Instruction(None, 0),
    1: Instruction(operator.add, 3),
    2: Instruction(operator.mul, 3),
    99: Instruction(EndProgram, 0),
}