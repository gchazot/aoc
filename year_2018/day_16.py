from __future__ import print_function
import copy
import operator
import unittest

from aoc_utils.data import data_lines


class TestProcessor(unittest.TestCase):
    def test_init_with_registers(self):
        p = Processor([4, 3, 2, 1])
        self.assertListEqual(p.registers, [4, 3, 2, 1])

    def test_addr(self):
        p = Processor([1, 10, 100, 1000])
        p.addr(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 11, 1000])
        p.addr(2, 1, 0)
        self.assertListEqual(p.registers, [21, 10, 11, 1000])

    def test_addi(self):
        p = Processor([1, 10, 100, 1000])
        p.addi(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 2, 1000])
        p.addi(2, 1, 0)
        self.assertListEqual(p.registers, [3, 10, 2, 1000])

    def test_mulr(self):
        p = Processor([1, 10, 100, 1000])
        p.mulr(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 10, 1000])
        p.mulr(2, 1, 0)
        self.assertListEqual(p.registers, [100, 10, 10, 1000])

    def test_muli(self):
        p = Processor([1, 10, 100, 1000])
        p.muli(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 1, 1000])
        p.muli(2, 1, 0)
        self.assertListEqual(p.registers, [1, 10, 1, 1000])

    def test_banr(self):
        p = Processor([1, 10, 100, 1000])
        p.banr(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 0, 1000])
        p.banr(2, 1, 0)
        self.assertListEqual(p.registers, [0, 10, 0, 1000])

    def test_bani(self):
        p = Processor([1, 10, 100, 1000])
        p.bani(0, 1, 2)
        self.assertListEqual(p.registers, [1, 10, 1, 1000])
        p.bani(2, 1, 0)
        self.assertListEqual(p.registers, [1, 10, 1, 1000])

    def test_parse_sample(self):
        sample = [
            'Before: [3, 2, 1, 1]',
            '9 2 1 2',
            'After:  [3, 2, 2, 1]',
        ]
        self.assertListEqual(parse_sample(sample), [
            [3, 2, 1, 1],
            [9, 2, 1, 2],
            [3, 2, 2, 1],
        ])

    def test_matching_instructions(self):
        sample = [
            'Before: [3, 2, 1, 1]',
            '9 2 1 2',
            'After:  [3, 2, 2, 1]',
        ]

        self.assertSetEqual(
            {"mulr", "addi", "seti"},
            set(matching_instructions(sample))
        )

    def test_options_per_sample(self):
        all_lines = data_lines(2018, "day_16_A_mine.txt")
        samples = list(parse_samples(all_lines))
        self.assertEqual(779, len(samples))

        all_matches = map(matching_instructions, samples)
        big_matches = filter(lambda matches: len(list(matches)) >= 3, all_matches)
        self.assertEqual(531, len(big_matches))

    def test_guess_instruction_ids(self):
        all_lines = data_lines(2018, "day_16_A_mine.txt")

        self.assertDictEqual(
            {
                0: 'banr', 1: 'addr', 2: 'eqri', 3: 'setr',
                4: 'gtrr', 5: 'bori', 6: 'gtir', 7: 'seti',
                8: 'borr', 9: 'bani', 10: 'eqir', 11: 'eqrr',
                12: 'gtri', 13: 'addi', 14: 'muli', 15: 'mulr'
            },
            find_instruction_ids(all_lines),
        )

    def test_execute_instructions(self):
        samples_lines = data_lines(2018, "day_16_A_mine.txt")
        instructions_mapping = find_instruction_ids(samples_lines)

        processor = Processor(registers=[0, 0, 0, 0])
        for program_line in data_lines(2018, "day_16_B_mine.txt"):
            instruction = list(map(int, program_line.split()))
            instruction_id = instruction[0]
            instruction_code = instructions_mapping[instruction_id]
            args = instruction[1:]
            getattr(processor, instruction_code)(*args)

        self.assertListEqual([649, 3, 2, 1], processor.registers)


def find_instruction_ids(all_lines):
    samples = list(parse_samples(all_lines))
    all_matches = list(map(lambda sample: list(matching_instructions(sample)), samples))
    operation_ids = [parse_sample(sample)[1][0] for sample in samples]
    instruction_ids = {}
    while len(instruction_ids) < 16:
        for i, matches in enumerate(all_matches):
            sample = samples[i]
            if sample[1][0] in instruction_ids:
                continue
            possible_matches = [
                match for match in matches
                if match not in instruction_ids.values()
            ]
            if len(possible_matches) == 1:
                id = operation_ids[i]
                code = possible_matches[0]
                final = instruction_ids.setdefault(id, code)
                if final != code:
                    print("Caca {} != {}", final, code)
    return instruction_ids


def parse_samples(all_lines):
    sample = []
    for line in all_lines:
        clean_line = line.strip()
        if len(clean_line) > 0:
            sample.append(clean_line)
        if len(sample) < 3:
            continue
        yield sample
        sample = []


def parse_sample(input_lines):
    before = input_lines[0][9:-1].replace(",", "")
    instruction = input_lines[1]
    after = input_lines[2][9:-1].replace(",", "")

    return [
        [int(num) for num in line.split()]
        for line in [before, instruction, after]
    ]


def matching_instructions(sample):
    before, instruction, after = parse_sample(sample)
    instruction_parameters = instruction[1:]
    for code in Processor.all_codes:
        processor = Processor(before)
        getattr(processor, code)(*instruction_parameters)
        if processor.registers == after:
            yield code


class Processor(object):
    all_codes = []

    def __init__(self, registers):
        self.registers = copy.copy(registers)

    @classmethod
    def add_operation(cls, code, method):
        setattr(cls, code, method)
        Processor.all_codes.append(code)

    @classmethod
    def register_register_operation(cls, code, op):
        def inner(self, a, b, c):
            self.registers[c] = op(self.registers[a], self.registers[b])
        cls.add_operation(code, inner)

    @classmethod
    def register_immediate_operation(cls, code, op):
        def inner(self, a, b, c):
            self.registers[c] = op(self.registers[a], b)
        cls.add_operation(code, inner)

    @classmethod
    def immediate_register_operation(cls, code, op):
        def inner(self, a, b, c):
            self.registers[c] = op(a, self.registers[b])
        cls.add_operation(code, inner)


def create_instruction_set():
    instructions = {
        "add": operator.add,
        "mul": operator.mul,
        "ban": operator.and_,
        "bor": operator.or_,
        "gtr": lambda a, b: 1 if a > b else 0,
        "eqr": lambda a, b: 1 if a == b else 0,
    }

    for code, op in instructions.items():
        Processor.register_register_operation(code + "r", op)
        Processor.register_immediate_operation(code + "i", op)

    Processor.immediate_register_operation("gtir", instructions["gtr"])
    Processor.immediate_register_operation("eqir", instructions["eqr"])

    Processor.immediate_register_operation("seti", lambda a, _b: a)
    Processor.register_immediate_operation("setr", lambda a, _b: a)


create_instruction_set()
