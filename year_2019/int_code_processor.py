import inspect


class EndProgram(Exception):
    pass


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
