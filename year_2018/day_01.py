from __future__ import print_function
from aoc_utils.data import data_text
import itertools
import unittest


class TestFinalFrequency(unittest.TestCase):
    def test_examples(self):
        self.assertEqual(3, final_frequency([1, -2, 3, 1]))
        self.assertEqual(3, final_frequency([1, 1, 1]))
        self.assertEqual(0, final_frequency([1, 1, -2]))
        self.assertEqual(-6, final_frequency([-1, -2, -3]))


def final_frequency(seq):
    return sum(seq)


class TestParseSequence(unittest.TestCase):
    def test_example(self):
        seq = "\n".join(["+1", "-2", "+3", "+1"])
        self.assertListEqual([1, -2, 3, 1], parse_sequence(seq))


def parse_sequence(seq_text):
    seq_elements = seq_text.split("\n")
    return list(map(int, seq_elements))


class TestFullProblem(unittest.TestCase):
    def test_first_mine(self):
        seq = read_frequencies_sequence()
        self.assertEqual(510, final_frequency(seq))

    def test_second_mine(self):
        seq = read_frequencies_sequence()
        self.assertEqual(69074, first_repeated_frequency(seq))


def read_frequencies_sequence():
    seq_file = data_text(2018, "day_01_mine.txt")
    seq = parse_sequence(seq_file)
    return seq


class TestFirstRepeatedFrequency(unittest.TestCase):
    def test_example(self):
        self.assertEqual(2, first_repeated_frequency([1, -2, 3, 1]))
        self.assertEqual(0, first_repeated_frequency([1, -1]))
        self.assertEqual(10, first_repeated_frequency([3, 3, 4, -2, -4]))
        self.assertEqual(5, first_repeated_frequency([-6, 3, 8, 5, -6]))
        self.assertEqual(14, first_repeated_frequency([7, 7, -2, -7, -4]))


def first_repeated_frequency(seq):
    current = 0
    seen = {0}
    for increment in itertools.cycle(seq):
        current += increment
        if current in seen:
            return current
        seen.add(current)
