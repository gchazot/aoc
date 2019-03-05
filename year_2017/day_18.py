import contextlib
import threading
import time
import unittest

try:
    import Queue as queue
except ImportError:
    import queue

try:
    from mock import MagicMock
except ImportError:
    from unittest.mock import MagicMock

from aoc_utils.data import data_lines

SPECIAL_SOUND_REGISTER = u"__SPECIAL_SOUND__"


class RecoveredFrequency(Exception):
    def __init__(self, frequency):
        self.frequency = frequency


class WaitingForInput(Exception):
    pass


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


class SendInstruction(InstructionBase):
    def __init__(self, registers, destination_queue):
        super(SendInstruction, self).__init__(registers)
        self.destination_queue = destination_queue

    def execute(self, register_or_value):
        value = self.get_register_or_value(register_or_value)
        self.destination_queue.put(value)


class TestSendInstruction(InstructionTestBase):
    def test_enqueues_item_in_destination(self):
        destination_queue = MagicMock()
        instruction = SendInstruction(self.registers, destination_queue)

        instruction.execute("123")
        destination_queue.put.assert_called_once_with(123)

        destination_queue.put.reset_mock()

        instruction.execute("456")
        destination_queue.put.assert_called_once_with(456)


class ReceiveInstruction(InstructionBase):
    def __init__(self, registers, source_queue):
        super(ReceiveInstruction, self).__init__(registers)
        self.source_queue = source_queue

    def execute(self, register):
        try:
            value = self.source_queue.get_nowait()
            self.set_register(register, value)
            self.source_queue.task_done()
            return 1
        except queue.Empty:
            raise WaitingForInput()


class TestReceiveInstruction(InstructionTestBase):
    def test_read_item_from_source(self):
        source_queue = MagicMock()
        source_queue.empty.return_value = False
        source_queue.get_nowait.side_effect = [123, 456, 789]

        instruction = ReceiveInstruction(self.registers, source_queue)

        self.assertEqual(1, instruction.execute("reg_a"))
        self.check_register("reg_a", 123)
        source_queue.get_nowait.assert_called_once()

        source_queue.get_nowait.reset_mock()

        self.assertEqual(1, instruction.execute("reg_b"))
        self.check_register("reg_b", 456)
        source_queue.get_nowait.assert_called_once()

    def test_holds_on_empty_source(self):
        source_queue = MagicMock()
        source_queue.empty.return_value = True
        source_queue.get_nowait.side_effect = queue.Empty

        instruction = ReceiveInstruction(self.registers, source_queue)

        self.assertRaises(WaitingForInput, instruction.execute, ("reg_a",))


class InstructionProcessor:
    def __init__(self, processor_id, instructions, code_lines):
        self._registers = {"p": processor_id}
        self._instructions = {
            code: Instruction(self._registers) for code, Instruction in instructions.items()
        }

        self._code_lines = code_lines
        self._end_index = len(self._code_lines)
        self._next_line = 0

        self._waiting_cycles = 0

    def run(self):
        while self.next_line_is_valid():
            self.exec_next_line()

    def is_waiting(self):
        return self._waiting_cycles > 0

    def next_line_is_valid(self):
        return 0 <= self._next_line < self._end_index

    def exec_next_line(self):
        line = self._code_lines[self._next_line]
        offset = self._exec_one_line(line)
        if offset is None:
            offset = 1
        self._next_line += offset

    def _exec_one_line(self, code_line):
        command, args = self._parse_code_line(code_line)

        try:
            instruction = self._instructions[command]
            result = instruction.execute(*args)
            self._waiting_cycles = 0
            return result
        except WaitingForInput:
            self._waiting_cycles += 1
            return 0
        except KeyError:
            raise RuntimeError(u"Invalid instruction: {}".format(code_line))

    @staticmethod
    def _parse_code_line(code_line):
        items = code_line.split()
        return items[0], items[1:]


class TestInstructionProcessor(unittest.TestCase):
    def setUp(self):
        mock_instructions = {
            "cmd1": MagicMock(),
            "cmd2": MagicMock(),
            "cmd3": MagicMock(),
        }
        code = (
            u"cmd1",
            u"cmd2",
            u"cmd3",
        )
        self.test_processor = InstructionProcessor(0, mock_instructions, code)
        self.test_instructions = self.test_processor._instructions
        for instruction in self.test_instructions.values():
            instruction.execute.return_value = None

    def test_initialises_instructions(self):
        processor = InstructionProcessor(0, sound_instructions, [])
        for instruction in processor._instructions.values():
            self.assertEqual(processor._registers, instruction._registers)

    def test_exec_one_line_calls_correct_instruction(self):
        self.test_processor._exec_one_line(u"cmd2 a b")
        for command, instruction in self.test_instructions.items():
            if command == "cmd2":
                instruction.execute.assert_called_once_with("a", "b")
            else:
                instruction.execute.assert_not_called()
            instruction.execute.reset_mock()

        self.test_processor._exec_one_line(u"cmd3 x y")
        for command, instruction in self.test_instructions.items():
            if command == "cmd3":
                instruction.execute.assert_called_once_with("x", "y")
            else:
                instruction.execute.assert_not_called()
            instruction.execute.reset_mock()

    def test_exec_one_line_fails_for_invalid_instruction(self):
        self.assertRaises(RuntimeError, self.test_processor._exec_one_line, "inval i d")

    def test_exec_program_runs_all_instructions(self):
        self.test_processor.run()

        for _command, instruction in self.test_instructions.items():
            instruction.execute.assert_called_once()

    def test_exec_program_jumps_instructions(self):
        self.test_instructions["cmd1"].execute.return_value = 2

        self.test_processor.run()

        self.test_instructions["cmd1"].execute.assert_called_once()
        self.test_instructions["cmd2"].execute.assert_not_called()
        self.test_instructions["cmd3"].execute.assert_called_once()

    def test_exec_program_jumps_backwards(self):
        self.test_instructions["cmd1"].execute.return_value = 2
        self.test_instructions["cmd2"].execute.return_value = 2
        self.test_instructions["cmd3"].execute.return_value = -1

        self.test_processor.run()

        self.test_instructions["cmd1"].execute.assert_called_once()
        self.test_instructions["cmd3"].execute.assert_called_once()
        self.test_instructions["cmd2"].execute.assert_called_once()


class InstructionProcessorThread(threading.Thread):
    def __init__(self, processor_id, instructions, code_lines):
        super(InstructionProcessorThread, self).__init__()
        self.processor = InstructionProcessor(processor_id, instructions, code_lines)

        self._terminate = False
        self._execution_lock = threading.Lock()

    def run(self):
        while self.processor.next_line_is_valid():
            time.sleep(0)
            with self._execution_lock:
                if self._terminate:
                    return
                self.processor.exec_next_line()

    def pause(self):
        self._execution_lock.acquire()

    def resume(self):
        self._execution_lock.release()

    @contextlib.contextmanager
    def pause_context(self):
        self.pause()
        yield
        self.resume()

    def is_waiting(self):
        return self.processor.is_waiting()

    def terminate(self):
        self._terminate = True


def start_processor_thread(processor_id, instructions, code_lines):
    processor_thread = InstructionProcessorThread(processor_id, instructions, code_lines)
    processor_thread.start()

    return processor_thread


class TestInstructionProcessorThread(unittest.TestCase):
    def test_can_enter_wait_state(self):
        mock_waiting_instruction = MagicMock()

        instructions = {
            "wam": mock_waiting_instruction,
        }
        code = ("wam",)

        processor_thread = InstructionProcessorThread(0, instructions, code)
        processor_thread.processor._instructions["wam"].execute.side_effect = WaitingForInput

        processor_thread.start()
        time.sleep(0.01)  # Give processor a head start

        was_waiting = processor_thread.is_waiting()

        processor_thread.terminate()
        processor_thread.join()

        self.assertTrue(was_waiting) # Might fail because of race condition

    def test_can_be_paused_and_resumed(self):
        mock_waiting_instruction = MagicMock()

        instructions = {
            "wms": mock_waiting_instruction,
        }
        code = ["wms"]

        def sleep_a_bit():
            time.sleep(0.01)
            raise WaitingForInput()

        processor_thread = InstructionProcessorThread(0, instructions, code)
        wait_instruction = processor_thread.processor._instructions["wms"]
        wait_instruction.execute.side_effect = sleep_a_bit

        processor_thread.start()
        time.sleep(0.01)  # Give processor a head start

        with processor_thread.pause_context():
            was_waiting = processor_thread.is_waiting()
            wait_instruction.execute.side_effect = lambda: None

        processor_thread.join()

        self.assertTrue(was_waiting)


sound_instructions = {
    "snd": SoundInstruction,
    "set": SetInstruction,
    "add": AddInstruction,
    "mul": MultiplyInstruction,
    "mod": ModuloInstruction,
    "jgz": JumpInstruction,
    "rcv": RecoverInstruction,
}


def read_code_file(filename):
    return list(data_lines(2017, filename))


class TestSoundProcessor(unittest.TestCase):
    
    def test_example_1(self):
        code = read_code_file("day_18_example.txt")
        processor = InstructionProcessor(0, sound_instructions, code)

        with self.assertRaises(RecoveredFrequency) as assertion:
            processor.run()
        frequency = assertion.exception.frequency
        self.assertEqual(4, frequency)

    def test_mine_1(self):
        code = read_code_file("day_18_mine.txt")
        processor = InstructionProcessor(0, sound_instructions, code)

        with self.assertRaises(RecoveredFrequency) as assertion:
            processor.run()
        frequency = assertion.exception.frequency
        self.assertEqual(7071, frequency)


def make_communicating_instruction(instruction_class, communication_queue):
    class CommunicatingInstruction(instruction_class):
        def __init__(self, registers):
            super(CommunicatingInstruction, self).__init__(registers, communication_queue)

    return CommunicatingInstruction


class CountingQueue(queue.Queue):
    def __init__(self, *args, **kwargs):
        queue.Queue.__init__(self, *args, **kwargs)
        self.count = 0

    def put(self, *args, **kwargs):
        queue.Queue.put(self, *args, **kwargs)
        self.count += 1


class TestCommunicatingProcessors(unittest.TestCase):
    @staticmethod
    def start_communication_processor_thread(processor_id, code, input_queue, output_queue):
        processor_instructions = {
            "set": SetInstruction,
            "add": AddInstruction,
            "mul": MultiplyInstruction,
            "mod": ModuloInstruction,
            "jgz": JumpInstruction,
            "snd": make_communicating_instruction(SendInstruction, output_queue),
            "rcv": make_communicating_instruction(ReceiveInstruction, input_queue),
        }
        return start_processor_thread(processor_id, processor_instructions, code)

    def test_simple_communication(self):
        code = ("snd 123", "rcv reg_a", "snd reg_a")

        to_a = queue.Queue()
        to_b = queue.Queue()

        to_a.put(456)

        thread_1 = self.start_communication_processor_thread(0, code, to_a, to_b)
        thread_1.join()

        self.assertEqual(123, to_b.get())
        self.assertEqual(456, to_b.get())

    def test_two_process_communication(self):
        code = ("snd 12", "rcv register")

        to_a = queue.Queue()
        to_b = queue.Queue()

        thread_1 = self.start_communication_processor_thread(0, code, to_a, to_b)
        thread_2 = self.start_communication_processor_thread(1, code, to_b, to_a)

        thread_1.join()
        thread_2.join()

        self.assertRaises(queue.Empty, to_a.get_nowait)
        self.assertRaises(queue.Empty, to_b.get_nowait)

    def test_two_process_deadlock(self):
        code = ("rcv register",)

        to_a = queue.Queue()
        to_b = queue.Queue()

        thread_1 = self.start_communication_processor_thread(0, code, to_a, to_b)
        thread_2 = self.start_communication_processor_thread(1, code, to_b, to_a)

        deadlock_found = False

        while True:
            time.sleep(0.01)
            if thread_1.is_waiting() and thread_2.is_waiting():
                with thread_1.pause_context(), thread_2.pause_context():
                    if thread_1.is_waiting() and thread_2.is_waiting():
                        thread_1.terminate()
                        thread_2.terminate()
                        deadlock_found = True
                        break

        thread_1.join()
        thread_2.join()

        self.assertTrue(deadlock_found)
        self.assertRaises(queue.Empty, to_a.get_nowait)
        self.assertRaises(queue.Empty, to_b.get_nowait)

    def test_example(self):
        code = read_code_file("day_18_example_2.txt")

        to_a = CountingQueue()
        to_b = CountingQueue()

        thread_1 = self.start_communication_processor_thread(0, code, to_a, to_b)
        thread_2 = self.start_communication_processor_thread(1, code, to_b, to_a)

        while True:
            time.sleep(0.01)
            if thread_1.is_waiting() and thread_2.is_waiting():
                with thread_1.pause_context(), thread_2.pause_context():
                    if thread_1.is_waiting() and thread_2.is_waiting():
                        thread_1.terminate()
                        thread_2.terminate()
                        break

        thread_1.join()
        thread_2.join()

        self.assertRaises(queue.Empty, to_a.get_nowait)
        self.assertRaises(queue.Empty, to_b.get_nowait)

        self.assertEqual(3, to_a.count)
        self.assertEqual(3, to_b.count)

    def test_mine(self):
        code = read_code_file("day_18_mine.txt")

        to_a = CountingQueue()
        to_b = CountingQueue()

        thread_1 = self.start_communication_processor_thread(0, code, to_a, to_b)
        thread_2 = self.start_communication_processor_thread(1, code, to_b, to_a)

        while True:
            time.sleep(0.01)
            if thread_1.is_waiting() and thread_2.is_waiting():
                with thread_1.pause_context(), thread_2.pause_context():
                    if thread_1.is_waiting() and thread_2.is_waiting() and to_a.empty() and to_b.empty():
                        thread_1.terminate()
                        thread_2.terminate()
                        break

        thread_1.join()
        thread_2.join()

        self.assertRaises(queue.Empty, to_a.get_nowait)
        self.assertRaises(queue.Empty, to_b.get_nowait)

        self.assertEqual(8001, to_a.count)
        self.assertEqual(8128, to_b.count)

