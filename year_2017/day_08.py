from unittest import TestCase
from collections import defaultdict
from aoc_utils import data_file
import re


class ProgramRunner:
    def __init__(self, filename):
        self.registers = defaultdict(lambda: 0)
        self.program = open(data_file(2017, filename))
        self.max_register_ever = 0

    def find_largest_register(self):
        self.exec_program()
        return self.max_register()

    def exec_program(self):
        for line in self.program.readlines():
            instruction = rewrite_instruction(line)
            exec(instruction, {}, self.registers)
            self.max_register_ever = max(self.max_register_ever, self.max_register())

    def max_register(self):
        return max(self.registers.values())


line_pattern = re.compile("^(?P<reg_1>\S+) (?P<cmd>\S+) (?P<value>\S+) if (?P<reg_2>\S+) (?P<test>.+)$")


def rewrite_instruction(line):
    match = line_pattern.match(line)
    elements = match.groupdict()
    elements["command"] = elements["cmd"].replace("inc", "+=").replace("dec", "-=")
    return "if reg_{reg_2} {test}: reg_{reg_1} {command} {value}".format(**elements)


class TestProcessInstructions(TestCase):
    def test_rewrite_instruction(self):
        self.assertEqual("if reg_a > 1: reg_b += 5", rewrite_instruction("b inc 5 if a > 1"))
        self.assertEqual("if reg_b < 5: reg_a += 1", rewrite_instruction("a inc 1 if b < 5"))
        self.assertEqual("if reg_a >= 1: reg_c -= -10", rewrite_instruction("c dec -10 if a >= 1"))
        self.assertEqual("if reg_c == 10: reg_c += -20", rewrite_instruction("c inc -20 if c == 10"))

    def test_find_largest_register_example(self):
        runner = ProgramRunner("day_08_example.txt")
        self.assertEqual(1, runner.find_largest_register())
        self.assertEqual(10, runner.max_register_ever)

    def test_find_largest_register_mine(self):
        runner = ProgramRunner("day_08_mine.txt")
        self.assertEqual(3880, runner.find_largest_register())
        self.assertEqual(5035, runner.max_register_ever)
