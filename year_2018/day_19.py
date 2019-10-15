from __future__ import print_function
import unittest
from aoc_utils.data import data_lines
from year_2018.day_16 import Processor


class TestJumpingProcessor(unittest.TestCase):
    def test_increments_instruction_pointer(self):
        processor = JumpingProcessor([0, 0, 0, 0], ip_register=3)
        self.assertEqual(3, processor.ip_register)
        self.assertEqual(0, processor.ip)

        processor.execute("addi", 0, 1, 2)
        self.assertListEqual([0, 0, 1, 0], processor.registers)
        self.assertEqual(1, processor.ip)

        processor.execute("addi", 0, 1, 2)
        self.assertListEqual([0, 0, 1, 1], processor.registers)
        self.assertEqual(2, processor.ip)

    def test_example_operations(self):
        processor = JumpingProcessor([0, 0, 0, 0, 0, 0], ip_register=0)

        processor.execute("seti", 5, 0, 1)
        self.assertListEqual([0, 5, 0, 0, 0, 0], processor.registers)
        self.assertEqual(1, processor.ip)

        processor.execute("seti", 6, 0, 2)
        self.assertListEqual([1, 5, 6, 0, 0, 0], processor.registers)
        self.assertEqual(2, processor.ip)

        processor.execute("addi", 0, 1, 0)
        self.assertListEqual([3, 5, 6, 0, 0, 0], processor.registers)
        self.assertEqual(4, processor.ip)

        processor.execute("setr", 1, 0, 0)
        self.assertListEqual([5, 5, 6, 0, 0, 0], processor.registers)
        self.assertEqual(6, processor.ip)

        processor.execute("seti", 9, 0, 5)
        self.assertListEqual([6, 5, 6, 0, 0, 9], processor.registers)
        self.assertEqual(7, processor.ip)


class JumpingProcessor(Processor):
    def __init__(self, registers, ip_register):
        super(JumpingProcessor, self).__init__(registers)
        self.ip_register = ip_register
        self.ip = 0

    def execute(self, code, *args):
        self.registers[self.ip_register] = self.ip
        super(JumpingProcessor, self).execute(code, *args)
        self.ip = self.registers[self.ip_register]
        self.ip += 1


class TestProgram(unittest.TestCase):
    @staticmethod
    def example_program():
        program = Program(
            num_registers=6,
            input_lines=[
                '#ip 0',
                'seti 5 0 1',
                'seti 6 0 2',
                'addi 0 1 0',
                'addr 1 2 3',
                'setr 1 0 0',
                'seti 8 0 4',
                'seti 9 0 5',
            ],
            processor_class=JumpingProcessor,
        )
        return program

    def test_parse(self):
        program = self.example_program()
        self.assertEqual(0, program.processor.ip_register)
        self.assertEqual(7, len(program.instructions))
        self.assertListEqual(['setr', 1, 0, 0], program.instructions[4])

    def test_execute_example(self):
        program = self.example_program()

        program.execute(10)

        self.assertListEqual([6, 5, 6, 0, 0, 9], program.processor.registers)
        self.assertEqual(7, program.processor.ip)

    @unittest.skip("Too slow")
    def test_mine(self):
        program = Program(
            num_registers=6,
            input_lines=data_lines(2018, "day_19_mine.txt"),
            processor_class=JumpingProcessor,
        )
        program.execute(10000000)
        self.assertEqual(2072, program.processor.registers[0])

    @unittest.skip("Wayyyyy toooo slow (would take years)")
    def test_mine_again(self):
        program = Program(
            num_registers=6,
            input_lines=data_lines(2018, "day_19_mine.txt"),
            initial_registers=[1, 0, 0, 0, 0, 0],
            processor_class=JumpingProcessor,
        )
        program.execute(34)

        self.assertEqual(2072, program.processor.registers[0])

    @staticmethod
    def decompiled(F):
        A = 0
        B = 1
        while True:
            C = 1
            while True:
                if B * C == F:
                    A = A + B
                C += 1
                if B * C > F:  # Originally if C > F
                    break
            B += 1
            if B > F:
                break
        return A

    def test_decompiled(self):
        self.assertEqual(2072, self.decompiled(876))

    @unittest.skip("Too slow")
    def test_decompiled_again(self):
        self.assertEqual(27578880, self.decompiled(10551276))


class Program:
    def __init__(self, num_registers, input_lines, initial_registers=None, processor_class=None):
        ip_register = None
        self.instructions = []
        for line in input_lines:
            terms = line.split()
            if ip_register is None:
                ip_register = int(terms[1])
            else:
                for i in range(1, len(terms)):
                    terms[i] = int(terms[i])
                self.instructions.append(terms)

        if initial_registers is None:
            initial_registers = [0 for _ in range(num_registers)]
        if processor_class is None:
            processor_class = JumpingProcessor

        self.processor = processor_class(initial_registers, ip_register)
        self.total_instructions = 0

    def execute(self, num_instructions, log=False):
        counter = 0
        while 0 <= self.processor.ip < len(self.instructions) and counter < num_instructions:
            ip = self.processor.ip
            instruction = self.instructions[ip]
            self.processor.execute(*instruction)
            if log:
                print(
                    f"{self.total_instructions:9d}({counter:4d}) {ip:2d} {str(instruction):20s}",
                    self.processor.registers
                )
            counter += 1
            self.total_instructions += 1
