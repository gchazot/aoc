from functools import cmp_to_key
from unittest import TestCase
from collections import defaultdict
from aoc_utils import data_file


class Decryptor:
    def __init__(self, description):
        fields = description.strip().split("-")
        self.crypted_name = fields[:-1]
        self.sector, self.checksum = fields[-1][:-1].split("[")
        self.valid = None
        self.check_valid()
        self.name = None

    def check_valid(self):
        letter_counts = defaultdict(lambda: 0)
        for letter in "".join(self.crypted_name):
            letter_counts[letter] += 1

        def cmp(a, b):
            return (a > b) - (a < b)

        def compare_counts(a, b):
            count_cmp = cmp(letter_counts[a], letter_counts[b])
            if count_cmp != 0:
                return count_cmp
            return -cmp(a, b)

        letters_ranked = sorted(letter_counts.keys(), key=cmp_to_key(compare_counts), reverse=True)
        self.valid = ("".join(letters_ranked)[:5] == self.checksum)

    def decrypt_name(self):
        new_name = []
        for word in self.crypted_name:
            new_word = []
            for letter in word:
                letter_code = ord('a') + (ord(letter) - ord('a') + int(self.sector)) % (ord('z') - ord('a') + 1)
                new_word.append(chr(letter_code))
            new_name.append("".join(new_word))
        self.name = " ".join(new_name)


def valid_room_sector(description):
    decrypt = Decryptor(description)
    if decrypt.valid:
        return int(decrypt.sector)
    return 0


def is_real_room(description):
    return valid_room_sector(description) != 0


def sum_valid_rooms_sectors(filename):
    with open(filename) as f:
        return sum(map(valid_room_sector, f.readlines()))


def find_objects_location(filename):
    with open(filename) as f:
        descriptions = [Decryptor(line) for line in f.readlines()]
        decryptors = [d for d in descriptions if d.valid]
        for d in decryptors:
            d.decrypt_name()
        north_pole = [int(d.sector) for d in decryptors if d.name == "northpole object storage"]
        return north_pole[0]


class TestDecoyRoomDetector(TestCase):
    def test_1_example_1(self):
        self.assertTrue(is_real_room("aaaaa-bbb-z-y-x-123[abxyz]"))

    def test_1_example_2(self):
        self.assertTrue(is_real_room("a-b-c-d-e-f-g-h-987[abcde]"))

    def test_1_example_3(self):
        self.assertTrue(is_real_room("not-a-real-room-404[oarel]"))

    def test_1_example_4(self):
        self.assertFalse(is_real_room("totally-real-room-200[decoy]"))

    def test_1_examples_file(self):
        self.assertEqual(1514, sum_valid_rooms_sectors(data_file(2016, "day_04_example.txt")))

    def test_1_mine(self):
        self.assertEqual(185371, sum_valid_rooms_sectors(data_file(2016, "day_04_mine.txt")))

    def test_2_mine(self):
        self.assertEqual(984, find_objects_location(data_file(2016, "day_04_mine.txt")))
