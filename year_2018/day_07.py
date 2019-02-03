from aoc_utils import data_file

import unittest


class TestInstructionManual(unittest.TestCase):
    def test_starts_empty(self):
        manual = InstructionManual()
        self.assertEqual(0, manual.num_instructions())
        self.assertEqual(0, manual.num_steps())

    def test_parse_instruction(self):
        parse = InstructionManual._parse_instruction
        self.assertEqual(('C', 'A'), parse('Step C must be finished before step A can begin.'))
        self.assertEqual(('C', 'F'), parse('Step C must be finished before step F can begin.'))
        self.assertEqual(('A', 'B'), parse('Step A must be finished before step B can begin.'))
        self.assertEqual(('A', 'D'), parse('Step A must be finished before step D can begin.'))
        self.assertEqual(('B', 'E'), parse('Step B must be finished before step E can begin.'))
        self.assertEqual(('D', 'E'), parse('Step D must be finished before step E can begin.'))
        self.assertEqual(('F', 'E'), parse('Step F must be finished before step E can begin.'))

    def test_add(self):
        manual = InstructionManual()

        manual.add('Step C must be finished before step A can begin.')
        self.assertEqual(1, manual.num_instructions())
        self.assertEqual(2, manual.num_steps())

        manual.add('Step C must be finished before step F can begin.')
        self.assertEqual(2, manual.num_instructions())
        self.assertEqual(3, manual.num_steps())

        manual.add('Step C must be finished before step A can begin.')
        manual.add('Step C must be finished before step F can begin.')
        self.assertEqual(2, manual.num_instructions())
        self.assertEqual(3, manual.num_steps())

    def test_find_sequence(self):
        manual = self._make_test_manual()
        self.assertEqual('CABDFE', "".join(manual.find_sequence()))

    def test_find_sequence_mine(self):
        manual = InstructionManual()

        with open(data_file(2018, 'day_07_mine.txt')) as f:
            for line in f.readlines():
                manual.add(line)

        self.assertEqual(26, manual.num_steps())
        self.assertEqual(101, manual.num_instructions())

        self.assertEqual("BDHNEGOLQASVWYPXUMZJIKRTFC", "".join(manual.find_sequence()))

    def test_step_time(self):
        self.assertEqual(61, InstructionManual._step_time('A'))
        self.assertEqual(62, InstructionManual._step_time('B'))
        self.assertEqual(86, InstructionManual._step_time('Z'))

    def test_calculate_time(self):
        class FakeInstructionManual(InstructionManual):
            @staticmethod
            def _step_time(step):
                return InstructionManual._step_time(step) - 60

        manual = self._make_test_manual(FakeInstructionManual)

        self.assertEqual(15, manual.calculate_time(num_workers=2))

    def test_calculate_time_mine(self):
        manual = InstructionManual()

        with open(data_file(2018, 'day_07_mine.txt')) as f:
            for line in f.readlines():
                manual.add(line)

        self.assertEqual(1107, manual.calculate_time(num_workers=5))

    def _make_test_manual(self, manual_class=None):
        if manual_class is None:
            manual = InstructionManual()
        else:
            manual = manual_class()
        manual.add('Step C must be finished before step A can begin.')
        manual.add('Step C must be finished before step F can begin.')
        manual.add('Step A must be finished before step B can begin.')
        manual.add('Step A must be finished before step D can begin.')
        manual.add('Step B must be finished before step E can begin.')
        manual.add('Step D must be finished before step E can begin.')
        manual.add('Step F must be finished before step E can begin.')
        return manual


class InstructionManual:
    def __init__(self):
        self._instructions = set()
        self._steps = set()

    def num_instructions(self):
        return len(self._instructions)

    def num_steps(self):
        return len(self._steps)

    def add(self, instruction_line):
        instruction = self._parse_instruction(instruction_line)
        self._instructions.add(instruction)
        self._steps.update(instruction)

    def find_sequence(self):
        ready_steps = self._steps.difference(b for a, b in self._instructions)
        done_steps = set()

        while len(ready_steps) > 0:
            step = min(ready_steps)

            ready_steps.remove(step)
            done_steps.add(step)
            yield step

            for next_step in self._get_required_by(step):
                if all(s in done_steps for s in self._get_requirements(next_step)):
                    ready_steps.add(next_step)

    def calculate_time(self, num_workers):
        workers = range(num_workers)
        workers_finishing_time = [0 for _ in workers]
        workers_task = [None for _ in workers]

        ready_steps = self._steps.difference(b for a, b in self._instructions)
        done_steps = set()

        current_time = 0
        while len(done_steps) < self.num_steps():
            for worker in workers:
                if len(ready_steps) == 0:
                    continue
                if workers_finishing_time[worker] > current_time:
                    continue

                step = min(ready_steps)
                ready_steps.remove(step)

                workers_task[worker] = step
                step_time = self._step_time(step)
                workers_finishing_time[worker] = current_time + step_time

                break
            else:
                next_finishing_workers = []
                next_finishing_time = None
                for worker in workers:
                    if workers_task[worker] is None:
                        continue

                    worker_finishing_time = workers_finishing_time[worker]
                    if next_finishing_time is None or worker_finishing_time < next_finishing_time:
                        next_finishing_workers = [worker]
                        next_finishing_time = worker_finishing_time
                    elif next_finishing_time == worker_finishing_time:
                        next_finishing_workers.append(worker)

                current_time = next_finishing_time

                for worker in next_finishing_workers:
                    step = workers_task[worker]
                    done_steps.add(step)
                    workers_task[worker] = None

                    for next_step in self._get_required_by(step):
                        if all(s in done_steps for s in self._get_requirements(next_step)):
                            ready_steps.add(next_step)

        return max(workers_finishing_time)

    @staticmethod
    def _parse_instruction(instruction_line):
        words = instruction_line.split()
        return words[1], words[7]

    @staticmethod
    def _step_time(step):
        return 61 + ord(step) - ord('A')

    def _get_requirements(self, step):
        return [a for a, b in self._instructions if b == step]

    def _get_required_by(self, step):
        return [b for a, b in self._instructions if a == step]
