import unittest
try:
    from mock import MagicMock
except ModuleNotFoundError:
    from unittest.mock import MagicMock
from aoc_utils import data_file


SPECIAL_SOUND_REGISTER = u"__SPECIAL_SOUND__"


class RecoveredFrequency(Exception):
    def __init__(self, frequency):
        self.frequency = frequency


class InstructionBase(object):
    def __init__(self, registers):
        self._registers = registers

    def execute(self, *args):
        raise NotImplementedError

    def get_register_or_value(self, arg):
        try:
            return int(arg)
        except ValueError:
            return self._registers.get(arg, 0)

    def set_register(self, target, value):
        self._registers[target] = value


class TestInstructionBase(unittest.TestCase):
    def test_exec_not_implemented(self):
        fake_registers = {}
        instruction = InstructionBase(fake_registers)

        self.assertRaises(NotImplementedError, instruction.execute)
        self.assertRaises(NotImplementedError, instruction.execute, 12)
        self.assertRaises(NotImplementedError, instruction.execute, 12, 34)

    def test_get_register_or_value(self):
        registers = {"reg_a": 12, "reg_b": 34}

        instruction_a = InstructionBase(registers)
        self.assertEqual(12, instruction_a.get_register_or_value(u"reg_a"))

        instruction_b = InstructionBase(registers)
        self.assertEqual(34, instruction_b.get_register_or_value(u"reg_b"))

        instruction_z = InstructionBase(registers)
        self.assertEqual(0, instruction_z.get_register_or_value(u"reg_z"))

        instruction_42 = InstructionBase(registers)
        self.assertEqual(42, instruction_42.get_register_or_value(u"42"))

        instruction_minus_12 = InstructionBase(registers)
        self.assertEqual(-12, instruction_minus_12.get_register_or_value(u"-12"))

    def test_set_register(self):
        registers = {"reg_a": 12, "reg_b": 34}

        instruction_a = InstructionBase(registers)
        instruction_a.set_register(u"reg_a", 56)
        self.assertEqual(56, registers.get(u"reg_a"))

        instruction_b = InstructionBase(registers)
        instruction_b.set_register(u"reg_b", 78)
        self.assertEqual(78, registers.get(u"reg_b"))

        instruction_z = InstructionBase(registers)
        instruction_z.set_register(u"reg_z", 90)
        self.assertEqual(90, registers.get(u"reg_z"))


class InstructionTestBase(unittest.TestCase):
    def setUp(self):
        self.registers = {"reg_a": 12, "reg_b": 34, "reg_0": 0}

    def check_register(self, register, expected_value):
        register_value = self.registers.get(register)
        self.assertEqual(register_value, expected_value)


class SoundInstruction(InstructionBase):
    def __init__(self, registers):
        super(SoundInstruction, self).__init__(registers)

    def execute(self, register_or_value):
        value = self.get_register_or_value(register_or_value)
        self.set_register(SPECIAL_SOUND_REGISTER, value)


class TestSoundInstruction(InstructionTestBase):
    def test_sound_register_is_initially_empty(self):
        self.check_register(SPECIAL_SOUND_REGISTER, None)

    def test_exec(self):
        instruction = SoundInstruction(self.registers)

        instruction.execute(u"123")
        self.check_register(SPECIAL_SOUND_REGISTER, 123)

        instruction.execute(u"456")
        self.check_register(SPECIAL_SOUND_REGISTER, 456)


class SetInstruction(InstructionBase):
    def __init__(self, registers):
        super(SetInstruction, self).__init__(registers)

    def execute(self, target_register, register_or_value):
        value = self.calculate_result(target_register, register_or_value)
        self.set_register(target_register, value)

    def calculate_result(self, target_register, register_or_value):
        return self.get_register_or_value(register_or_value)


class TestSetInstruction(InstructionTestBase):
    def test_exec(self):
        instruction = SetInstruction(self.registers)

        instruction.execute(u"reg_a", u"21")
        self.check_register(u"reg_a", 21)
        self.check_register(u"reg_b", 34)

        instruction.execute(u"reg_b", u"56")
        self.check_register(u"reg_a", 21)
        self.check_register(u"reg_b", 56)

        instruction.execute(u"reg_a", u"reg_b")
        self.check_register(u"reg_a", 56)
        self.check_register(u"reg_b", 56)


class AddInstruction(SetInstruction):
    def calculate_result(self, target_register, register_or_value):
        target_value = self.get_register_or_value(target_register)
        multiplier_value = self.get_register_or_value(register_or_value)
        return target_value + multiplier_value


class MultiplyInstruction(SetInstruction):
    def calculate_result(self, target_register, register_or_value):
        target_value = self.get_register_or_value(target_register)
        multiplier_value = self.get_register_or_value(register_or_value)
        return target_value * multiplier_value


class ModuloInstruction(SetInstruction):
    def calculate_result(self, target_register, register_or_value):
        target_value = self.get_register_or_value(target_register)
        multiplier_value = self.get_register_or_value(register_or_value)
        return target_value % multiplier_value


class RecoverInstruction(InstructionBase):
    def __init__(self, registers):
        super(RecoverInstruction, self).__init__(registers)

    def execute(self, register_or_value):
        value = self.get_register_or_value(register_or_value)
        if value is not None and value != 0:
            frequency = self.get_register_or_value(SPECIAL_SOUND_REGISTER)
            raise RecoveredFrequency(frequency)


class TestRecoverInstruction(InstructionTestBase):
    def test_0_arg_does_not_recover(self):
        instruction = RecoverInstruction(self.registers)

        instruction.execute(u"0")
        instruction.execute(u"reg_0")

    def test_invalid_arg_does_not_recover_sound(self):
        instruction = RecoverInstruction(self.registers)

        instruction.execute(u"")
        instruction.execute(u"reg_99")

    def test_recovers_None_when_no_sound_was_played(self):
        instruction = RecoverInstruction(self.registers)

        with self.assertRaises(RecoveredFrequency) as assertion:
            instruction.execute(u"1")
        frequency = assertion.exception.frequency
        self.assertEqual(0, frequency)

        with self.assertRaises(RecoveredFrequency) as assertion:
            instruction.execute(u"reg_a")
        frequency = assertion.exception.frequency
        self.assertEqual(0, frequency)

    def test_recovers_value_when_sound_was_played(self):
        self.registers[SPECIAL_SOUND_REGISTER] = 21
        instruction = RecoverInstruction(self.registers)

        with self.assertRaises(RecoveredFrequency) as assertion:
            instruction.execute(u"1")
        frequency = assertion.exception.frequency
        self.assertEqual(21, frequency)

        with self.assertRaises(RecoveredFrequency) as assertion:
            instruction.execute(u"reg_a")
        frequency = assertion.exception.frequency
        self.assertEqual(21, frequency)


class JumpInstruction(InstructionBase):
    def __init__(self, registers):
        super(JumpInstruction, self).__init__(registers)

    def execute(self, check_register_or_value, offset_register_or_value):
        check_value = self.get_register_or_value(check_register_or_value)
        offset_value = self.get_register_or_value(offset_register_or_value)
        if check_value is not None and check_value > 0:
            return offset_value


class TestJumpInstruction(InstructionTestBase):
    def test_does_not_jump_if_value_less_or_equal_to_zero(self):
        instruction = JumpInstruction(self.registers)

        self.assertEqual(None, instruction.execute("0", "reg_a"))
        self.assertEqual(None, instruction.execute("-5", "reg_a"))
        self.assertEqual(None, instruction.execute("reg_0", "reg_a"))

        self.registers["reg_minus_2"] = -2
        self.assertEqual(None, instruction.execute("reg_minus_2", "reg_a"))

    def test_jumps_if_value_greater_than_0(self):
        instruction = JumpInstruction(self.registers)

        self.assertEqual(123, instruction.execute("1", "123"))
        self.assertEqual(456, instruction.execute("234", "456"))
        self.assertEqual(12, instruction.execute("reg_a", "12"))
        self.assertEqual(34, instruction.execute("reg_b", "34"))


class InstructionProcessor:
    def __init__(self, instructions):
        self._registers = {}
        self._instructions = {
            code: Instruction(self._registers) for code, Instruction in instructions.items()
        }

    def exec_program(self, code_lines):
        program = list(code_lines)
        index = 0
        end = len(program)

        while 0 <= index < end:
            line = program[index]
            offset = self.exec_one_line(line)
            if offset is None:
                offset = 1
            index += offset

    def exec_one_line(self, code_line):
        items = code_line.split()
        command = items[0]
        args = items[1:]

        try:
            instruction = self._instructions[command]
            return instruction.execute(*args)
        except KeyError:
            raise RuntimeError(u"Invalid instruction: {}".format(code_line))


default_instructions = {
    "snd": SoundInstruction,
    "set": SetInstruction,
    "add": AddInstruction,
    "mul": MultiplyInstruction,
    "mod": ModuloInstruction,
    "jgz": JumpInstruction,
    "rcv": RecoverInstruction,
}


class TestInstructionProcessor(unittest.TestCase):
    def setUp(self):
        mock_instructions = {
            "cmd1": MagicMock(),
            "cmd2": MagicMock(),
            "cmd3": MagicMock(),
        }
        self.test_processor = InstructionProcessor(mock_instructions)
        self.test_instructions = self.test_processor._instructions
        for instruction in self.test_instructions.values():
            instruction.execute.return_value = None

    def test_initialises_instructions(self):
        processor = InstructionProcessor(default_instructions)
        for instruction in processor._instructions.values():
            self.assertEqual(processor._registers, instruction._registers)

    def test_exec_one_line_calls_correct_instruction(self):
        self.test_processor.exec_one_line(u"cmd2 a b")
        for command, instruction in self.test_instructions.items():
            if command == "cmd2":
                instruction.execute.assert_called_once_with("a", "b")
            else:
                instruction.execute.assert_not_called()
            instruction.execute.reset_mock()

        self.test_processor.exec_one_line(u"cmd3 x y")
        for command, instruction in self.test_instructions.items():
            if command == "cmd3":
                instruction.execute.assert_called_once_with("x", "y")
            else:
                instruction.execute.assert_not_called()
            instruction.execute.reset_mock()

    def test_exec_one_line_fails_for_invalid_instruction(self):
        self.assertRaises(RuntimeError, self.test_processor.exec_one_line, "inval i d")

    def test_exec_program_runs_all_instructions(self):
        code = (
            u"cmd1",
            u"cmd2",
            u"cmd3",
        )
        self.test_processor.exec_program(code)

        for _command, instruction in self.test_instructions.items():
            instruction.execute.assert_called_once()

    def test_exec_program_jumps_instructions(self):
        code = (
            u"cmd1",
            u"cmd2",
            u"cmd3",
        )
        self.test_instructions["cmd1"].execute.return_value = 2

        self.test_processor.exec_program(code)

        self.test_instructions["cmd1"].execute.assert_called_once()
        self.test_instructions["cmd2"].execute.assert_not_called()
        self.test_instructions["cmd3"].execute.assert_called_once()

    def test_exec_program_jumps_backwards(self):
        code = (
            u"cmd1",
            u"cmd2",
            u"cmd3",
        )
        self.test_instructions["cmd1"].execute.return_value = 2
        self.test_instructions["cmd2"].execute.return_value = 2
        self.test_instructions["cmd3"].execute.return_value = -1

        self.test_processor.exec_program(code)

        self.test_instructions["cmd1"].execute.assert_called_once()
        self.test_instructions["cmd3"].execute.assert_called_once()
        self.test_instructions["cmd2"].execute.assert_called_once()


class TestRealProcessor(unittest.TestCase):
    @staticmethod
    def read_code(filename):
        with open(data_file(2017, filename)) as code:
            return code.readlines()
    
    def test_example_1(self):
        code = self.read_code("day_18_example.txt")
        processor = InstructionProcessor(default_instructions)

        with self.assertRaises(RecoveredFrequency) as assertion:
            processor.exec_program(code)
        frequency = assertion.exception.frequency
        self.assertEqual(4, frequency)

    def test_mine_1(self):
        code = self.read_code("day_18_mine.txt")
        processor = InstructionProcessor(default_instructions)

        with self.assertRaises(RecoveredFrequency) as assertion:
            processor.exec_program(code)
        frequency = assertion.exception.frequency
        self.assertEqual(7071, frequency)
