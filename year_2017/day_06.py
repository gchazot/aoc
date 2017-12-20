from unittest import TestCase
import math


def largest_index(iterable):
    max_value = None
    max_index = None
    for i, item in enumerate(iterable):
        if max_value is None or item > max_value:
            max_value = item
            max_index = i
    return max_index


def serialize(iterable):
    return ",".join(map(str, iterable))


def re_balance_from(banks, start):
    size = len(banks)

    pool = banks[start]
    banks[start] = 0

    i = 0
    while pool > 0:
        count = math.ceil(pool / (size - i))
        i += 1
        n = (start + i) % size
        banks[n] += count
        pool -= count


def re_balance_banks(banks):
    seen_states = {}
    loops = 0
    while True:
        state = serialize(banks)
        if state in seen_states:
            return loops, loops - seen_states[state]
        seen_states[state] = loops
        loops += 1
        re_balance_from(banks, largest_index(banks))


class TestReallocation(TestCase):
    def test_1_example(self):
        self.assertEqual((5, 4), re_balance_banks([0, 2, 7, 0]))

    def test_1_mine(self):
        my_banks = list(map(int, "0 5 10 0 11 14 13 4 11 8 8 7 1 4 12 11".split()))
        self.assertEqual((7864, 1695), re_balance_banks(my_banks))
