from operator import xor
from unittest import TestCase

from aoc_utils import data_file


class KnotHash:
    def __init__(self, rope_length):
        self.length = rope_length
        self.rope = range(self.length)
        self.position = 0
        self.skip_size = 0

    def hash_v1(self, instructions):
        self.hash_string(instructions)
        return self.get_hash_v1()

    def hash_v2(self, seed):
        self.apply_instructions_v2(seed)
        return self.get_hash_v2()

    def get_hash_v1(self):
        return self.rope[0] * self.rope[1]

    def get_hash_v2(self):
        dense_hash = []
        for i in range(16):
            dense_hash.append(reduce(xor, self.rope[i * 16:(i+1) * 16]))

        def format_hex(integer):
            return "{:02x}".format(integer)

        return "".join(map(format_hex, dense_hash))

    def apply_instructions_v2(self, byte_stream):
        for _ in range(64):
            self.hash_bytes(byte_stream)
            self.apply_instructions([17, 31, 73, 47, 23])

    def hash_bytes(self, byte_stream):
        instructions = map(ord, byte_stream)
        self.apply_instructions(instructions)

    def hash_string(self, instructions):
        instructions = map(int, instructions.split(","))
        self.apply_instructions(instructions)

    def apply_instructions(self, instructions):
        for instruction in instructions:
            self.reverse_range(instruction)
            self.move_forward(instruction + self.skip_size)
            self.skip_size += 1

    def reverse_range(self, length):
        direct_range_length = min(length, self.length - self.position)
        direct_range_end = self.position + direct_range_length
        overflow_range_length = length - direct_range_length

        first_part = self.rope[self.position:direct_range_end]
        second_part = self.rope[0:overflow_range_length]
        new_range = list(reversed(first_part + second_part))

        self.rope[0:overflow_range_length] = new_range[direct_range_length:]
        self.rope[self.position:direct_range_end] = new_range[:direct_range_length]

    def move_forward(self, steps):
        self.position = (self.position + steps) % self.length


class TestRopeHash(TestCase):
    def test_example_1(self):
        self.assertEqual(12, KnotHash(5).hash_v1("3,4,1,5"))

    def test_1_mine(self):
        with open(data_file(2017, "day_10_mine.txt")) as f:
            self.assertEqual(9656, KnotHash(256).hash_v1(f.read()))

    def test_example_2(self):
        self.assertEqual("a2582a3a0e66e6e86e3812dcb672a272", KnotHash(256).hash_v2(""))
        self.assertEqual("33efeb34ea91902bb2f59c9920caa6cd", KnotHash(256).hash_v2("AoC 2017"))
        self.assertEqual("3efbe78a8d82f29979031a4aa0b16a9d", KnotHash(256).hash_v2("1,2,3"))
        self.assertEqual("63960835bcdc130f0b66d7ff4f6a5a8e", KnotHash(256).hash_v2("1,2,4"))

    def test_2_mine(self):
        with open(data_file(2017, "day_10_mine.txt")) as f:
            self.assertEqual("20b7b54c92bf73cf3e5631458a715149", KnotHash(256).hash_v2(f.read()))
