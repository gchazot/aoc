from unittest import TestCase

from aoc_utils import data_file


def open_marker(text, start):
    return text.find("(", start)


def close_marker(text, start):
    return text.find(")", start)


def next_marker(text, start):
    begin = open_marker(text, start)
    if begin < 0:
        return -1, -1
    end = close_marker(text, begin + 1)
    if end < 0:
        return -1, -1
    return begin + 1, end


def decompressed_size(text, recursive):
    size = 0
    i = 0
    while i < len(text):
        begin, end = next_marker(text, i)
        if begin < 0:
            size += len(text) - i
            break
        else:
            marker = text[begin:end]
            chars, times = map(int, marker.split("x"))
            size += begin - i - 1
            i = end + 1 + chars
            if recursive:
                size += decompressed_size(text[end + 1:i], recursive) * times
            else:
                size += chars * times
    return size


def decompress_file(filename, recursive):
    with open(data_file(2016, filename)) as f:
        text = f.read()
        return decompressed_size(text, recursive)


class TestDecompression(TestCase):
    def setUp(self):
        self.sample1 = "012(4x6)8(0x2)4"

    def test_look_for_open_marker(self):
        self.assertEqual(3, open_marker(self.sample1, 0))
        self.assertEqual(9, open_marker(self.sample1, 4))
        self.assertEqual(-1, open_marker(self.sample1, 10))

    def test_look_for_close_marker(self):
        self.assertEqual(7, close_marker(self.sample1, 0))
        self.assertEqual(13, close_marker(self.sample1, 8))
        self.assertEqual(-1, close_marker(self.sample1, 14))

    def test_find_next_marker(self):
        self.assertEqual((4,7), next_marker(self.sample1, 0))
        self.assertEqual((10,13), next_marker(self.sample1, 4))

    def test_decompressed_size_examples(self):
        self.assertEqual(6, decompressed_size("ADVENT", False))
        self.assertEqual(7, decompressed_size("A(1x5)BC", False))
        self.assertEqual(9, decompressed_size("(3x3)XYZ", False))
        self.assertEqual(11, decompressed_size("A(2x2)BCD(2x2)EFG", False))
        self.assertEqual(6, decompressed_size("(6x1)(1x3)A", False))
        self.assertEqual(18, decompressed_size("X(8x2)(3x3)ABCY", False))

    def test_decompressed_size_mine(self):
        self.assertEqual(112830, decompress_file("day_09_mine.txt", False))

    def test_decompressed_size_v2_examples(self):
        self.assertEqual(6, decompressed_size("ADVENT", True))
        self.assertEqual(7, decompressed_size("A(1x5)BC", True))
        self.assertEqual(9, decompressed_size("(3x3)XYZ", True))
        self.assertEqual(11, decompressed_size("A(2x2)BCD(2x2)EFG", True))
        self.assertEqual(3, decompressed_size("(6x1)(1x3)A", True))
        self.assertEqual(20, decompressed_size("X(8x2)(3x3)ABCY", True))
        self.assertEqual(241920, decompressed_size("(27x12)(20x12)(13x14)(7x10)(1x12)A", True))
        self.assertEqual(
            445,
            decompressed_size("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", True))

    def test_decompressed_size_v2_mine(self):
        self.assertEqual(10931789799, decompress_file("day_09_mine.txt", True))
