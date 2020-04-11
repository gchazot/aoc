import itertools
import unittest


class TestCounters(unittest.TestCase):
    def test_product(self):
        digits = list(range(10))
        options = itertools.product(digits, repeat=6)
        options_list = list(options)
        self.assertTupleEqual((5, 1, 2, 3, 3, 1), options_list[512331])

    @unittest.skip("Thit counter is too slow")
    def test_int_to_str(self):
        options_str = ["{0:0>6d}".format(i) for i in range(1000000)]
        options_list = [tuple(int(c) for c in option) for option in options_str]
        self.assertTupleEqual((5, 1, 2, 3, 3, 1), options_list[512331])


class TestPasswordChecker(unittest.TestCase):
    def test_examples(self):
        checker = PasswordChecker(0, 999999)
        self.assertTrue(checker.valid_password(parse(122345), max_repeat=None))
        self.assertTrue(checker.valid_password(parse(111123), max_repeat=None))
        self.assertFalse(checker.valid_password(parse(135679), max_repeat=None))
        self.assertTrue(checker.valid_password(parse(111111), max_repeat=None))
        self.assertFalse(checker.valid_password(parse(223450), max_repeat=None))
        self.assertFalse(checker.valid_password(parse(123789), max_repeat=None))

        self.assertTrue(checker.valid_password(parse(122345), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(111123), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(135679), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(111111), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(223450), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(123789), max_repeat=2))
        self.assertTrue(checker.valid_password(parse(112233), max_repeat=2))
        self.assertFalse(checker.valid_password(parse(123444), max_repeat=2))
        self.assertTrue(checker.valid_password(parse(111122), max_repeat=2))


    def test_count_valid_passwords(self):
        checker = PasswordChecker(123257, 647015)
        self.assertEqual(2220, checker.count_valid_passwords(max_repeat=None))
        self.assertEqual(1515, checker.count_valid_passwords(max_repeat=2))


class PasswordChecker:
    def __init__(self, minimum, maximum):
        self.minimum = parse(minimum)
        self.maximum = parse(maximum)

    def valid_password(self, password, max_repeat):
        if not max_repeat:
            max_repeat = len(password)

        if password < self.minimum or password > self.maximum:
            return False

        has_repeated = False
        previous = -1
        previous_count = 1
        for i in password:
            if i < previous:
                return False
            if i == previous:
                previous_count += 1
                if previous_count > max_repeat:
                    continue
            else:
                if 1 < previous_count <= max_repeat:
                    has_repeated = True
                previous_count = 1
            previous = i
        if 1 < previous_count <= max_repeat:
            has_repeated = True
        return has_repeated

    def count_valid_passwords(self, max_repeat):
        num_valid = 0

        digits = list(range(10))
        passwords = itertools.product(digits, repeat=6)
        for password in passwords:
            if self.valid_password(password, max_repeat):
                num_valid += 1
        return num_valid


def parse(password_num):
    clean_str = "{0:0>6d}".format(password_num)
    return tuple(int(c) for c in clean_str)
