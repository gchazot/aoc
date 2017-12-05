from unittest import TestCase


def no_same_words(phrase):
    all_words = phrase.split()
    unique_words = set(all_words)
    return len(all_words) == len(unique_words)


def count_letters(word):
    counts = [0 for _ in range(ord('z') - ord('a') + 1)]
    for c in word:
        counts[ord(c) - ord('a')] += 1
    return ''.join((chr(c) for c in counts))


def no_anagram(phrase):
    all_words = phrase.split()
    normalised_words = map(count_letters, all_words)
    unique_anagrams = set(normalised_words)
    return len(all_words) == len(unique_anagrams)


def count_valid(filename, policy):
    with open(filename) as f:
        return sum(map(policy, f.readlines()))


class TestPasswordValidator(TestCase):
    def test_no_same_word_1(self):
        self.assertTrue(no_same_words("aa bb cc dd ee"))

    def test_no_same_word_2(self):
        self.assertFalse(no_same_words("aa bb cc dd aa"))

    def test_no_same_word_3(self):
        self.assertTrue(no_same_words("aa bb cc dd aaa"))

    def test_no_same_word_mine(self):
        self.assertEqual(477, count_valid("data/day_04_1_mine.txt", no_same_words))

    def test_no_anagram_1(self):
        self.assertTrue(no_anagram("abcde fghij"))

    def test_no_anagram_2(self):
        self.assertFalse(no_anagram("abcde xyz ecdab"))

    def test_no_anagram_3(self):
        self.assertTrue(no_anagram("a ab abc abd abf abj"))

    def test_no_anagram_4(self):
        self.assertTrue(no_anagram("iiii oiii ooii oooi oooo"))

    def test_no_anagram_5(self):
        self.assertFalse(no_anagram("oiii ioii iioi iiio"))

    def test_no_anagram_mine(self):
        self.assertEqual(167, count_valid("data/day_04_1_mine.txt", no_anagram))