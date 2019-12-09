import inspect


class EndProgram(Exception):
    pass


class Instruction:
    def __init__(self, operation):
        self.operation = operation

    def __call__(self, instruction, memory):
        if self.operation == EndProgram:
            raise EndProgram
        elif self.operation is None:
            pass
        elif inspect.isbuiltin(self.operation) or inspect.isfunction(self.operation):
            a_index = instruction[1]
            b_index = instruction[2]
            c_index = instruction[3]
            memory[c_index] = self.operation(memory[a_index], memory[b_index])


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
            try:
                self.execute_instruction_at(self.instruction_pointer)
            except EndProgram:
                return
            self.instruction_pointer += self.instruction_size

    def execute_instruction_at(self, address):
        instruction = self.get_instruction(address)
        self.execute_instruction(instruction)

    def get_instruction(self, address):
        return self.memory[address:address + 4]

    def execute_instruction(self, instruction):
        operation_code = instruction[0]
        operation = self.instructions[operation_code]

        operation(instruction, self.memory)
