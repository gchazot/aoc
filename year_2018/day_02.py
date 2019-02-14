from collections import defaultdict
import unittest
from aoc_utils import data_lines


class TestIdProperties(unittest.TestCase):
    def test_has_two_same_letter(self):
        self.assertFalse(IdProperties("abcdef").has_two_same_letter())
        self.assertTrue(IdProperties("bababc").has_two_same_letter())
        self.assertTrue(IdProperties("abbcde").has_two_same_letter())
        self.assertFalse(IdProperties("abcccd").has_two_same_letter())
        self.assertTrue(IdProperties("aabcdd").has_two_same_letter())
        self.assertTrue(IdProperties("abcdee").has_two_same_letter())
        self.assertFalse(IdProperties("ababab").has_two_same_letter())

    def test_has_three_same_letter(self):
        self.assertFalse(IdProperties("abcdef").has_three_same_letter())
        self.assertTrue(IdProperties("bababc").has_three_same_letter())
        self.assertFalse(IdProperties("abbcde").has_three_same_letter())
        self.assertTrue(IdProperties("abcccd").has_three_same_letter())
        self.assertFalse(IdProperties("aabcdd").has_three_same_letter())
        self.assertFalse(IdProperties("abcdee").has_three_same_letter())
        self.assertTrue(IdProperties("ababab").has_three_same_letter())


class IdProperties(object):
    def __init__(self, word):
        self._word = word
        self._letter_counts = None

    def has_two_same_letter(self):
        counts = self._get_letter_counts()
        return 2 in counts.values()

    def has_three_same_letter(self):
        counts = self._get_letter_counts()
        return 3 in counts.values()

    def get_without_character_at(self, position):
        return "".join([self._word[:position], self._word[position+1:]])

    def _get_letter_counts(self):
        if self._letter_counts is None:
            self._letter_counts = defaultdict(lambda: 0)
            for letter in self._word:
                self._letter_counts[letter] += 1

        return self._letter_counts


class TestCheckSumOfList(unittest.TestCase):
    def test_example(self):
        self.assertEqual(12, checksum_of(
            ["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]))

    def test_mine(self):
        id_list = self.get_ids_list_mine()
        self.assertEqual(5904, checksum_of(id_list))

    def get_ids_list_mine(self):
        return list(data_lines(2018, "day_02_mine.txt"))


def checksum_of(id_list):
    props = list(IdProperties(id_text) for id_text in id_list)
    twos = len([prop for prop in props if prop.has_two_same_letter()])
    threes = len([prop for prop in props if prop.has_three_same_letter()])

    return twos * threes


class TestFindMatchingIds(unittest.TestCase):
    def test_get_without_character_at(self):
        id_props = IdProperties("abcde")
        self.assertEqual("bcde", id_props.get_without_character_at(0))
        self.assertEqual("acde", id_props.get_without_character_at(1))
        self.assertEqual("abde", id_props.get_without_character_at(2))
        self.assertEqual("abce", id_props.get_without_character_at(3))
        self.assertEqual("abcd", id_props.get_without_character_at(4))

    def test_make_list_without_character_at(self):
        id_list = ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]
        props = list(IdProperties(id_text) for id_text in id_list)

        self.assertListEqual(["bcde", "ghij", "lmno", "qrst", "guij", "xcye", "vxyz"],
                             make_list_without_character_at(props, 0))
        self.assertListEqual(["acde", "fhij", "kmno", "prst", "fuij", "acye", "wxyz"],
                             make_list_without_character_at(props, 1))
        self.assertListEqual(["abde", "fgij", "klno", "pqst", "fgij", "axye", "wvyz"],
                             make_list_without_character_at(props, 2))
        self.assertListEqual(["abce", "fghj", "klmo", "pqrt", "fguj", "axce", "wvxz"],
                             make_list_without_character_at(props, 3))
        self.assertListEqual(["abcd", "fghi", "klmn", "pqrs", "fgui", "axcy", "wvxy"],
                             make_list_without_character_at(props, 4))

    def test_get_duplicate_id(self):
        self.assertEqual(None, get_duplicate_id(
            ["bcde", "ghij", "lmno", "qrst", "guij", "xcye", "vxyz"]))
        self.assertEqual(None, get_duplicate_id(
            ["acde", "fhij", "kmno", "prst", "fuij", "acye", "wxyz"]))

        self.assertEqual("fgij", get_duplicate_id(
            ["abde", "fgij", "klno", "pqst", "fgij", "axye", "wvyz"]))

        self.assertEqual(None, get_duplicate_id(
            ["abce", "fghj", "klmo", "pqrt", "fguj", "axce", "wvxz"]))
        self.assertEqual(None, get_duplicate_id(
            ["abcd", "fghi", "klmn", "pqrs", "fgui", "axcy", "wvxy"]))

    def test_find_magic_id(self):
        self.assertEqual(None, find_magic_id(
            ["abcde", "fghik", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]))

        self.assertEqual("fgij", find_magic_id(
            ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]))

        id_list = list(data_lines(2018, "day_02_mine.txt"))
        self.assertEqual("jiwamotgsfrudclzbyzkhlrvp", find_magic_id(id_list))


def find_magic_id(id_list):
    props = [IdProperties(id_text.strip()) for id_text in id_list]

    for position in range(len(id_list[0])):
        chopped_ids = make_list_without_character_at(props, position)
        duplicate = get_duplicate_id(chopped_ids)
        if duplicate is not None:
            return duplicate


def get_duplicate_id(ids):
    seen = set()
    for id in ids:
        if id in seen:
            return id
        seen.add(id)


def make_list_without_character_at(id_props, position):
    return [prop.get_without_character_at(position) for prop in id_props]
