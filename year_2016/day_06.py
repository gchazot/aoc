from unittest import TestCase

from collections import defaultdict

from aoc_utils import data_file


class MessageDecryptor:
    def __init__(self):
        self.letter_occurrences = None

    def update(self, word):
        if self.letter_occurrences is None:
            self._init_occurrences(word)
        for i, letter in enumerate(word):
            self.letter_occurrences[i][letter] += 1

    def _init_occurrences(self, word):
        self.letter_occurrences = [defaultdict(lambda: 0) for _ in range(len(word))]

    def word(self, algorithm):
        return "".join(map(algorithm, self.letter_occurrences))


def most_occurrences(occurrences):
    best_letter = max(occurrences.items(), key=lambda letter_count: letter_count[1])
    return best_letter[0]


def least_occurrences(occurrences):
    best_letter = min(occurrences.items(), key=lambda letter_count: letter_count[1])
    return best_letter[0]


def decrypt_message(filename, algorithm):
    with open(data_file(2016, filename)) as f:
        decryptor = MessageDecryptor()
        for line in f.readlines():
            decryptor.update(line.strip())
        return decryptor.word(algorithm)


class TestDecryptMessage(TestCase):
    def test_1_example(self):
        self.assertEqual("easter", decrypt_message("day_06_example.txt", most_occurrences))

    def test_1_mine(self):
        self.assertEqual("liwvqppc", decrypt_message("day_06_mine.txt", most_occurrences))

    def test_2_example(self):
        self.assertEqual("advent", decrypt_message("day_06_example.txt", least_occurrences))

    def test_2_mine(self):
        self.assertEqual("caqfbzlh", decrypt_message("day_06_mine.txt", least_occurrences))
