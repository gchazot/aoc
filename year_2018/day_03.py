from __future__ import print_function

import collections
import re
import unittest

from aoc_utils import data_file


class TestClaimsList(unittest.TestCase):
    def test_starts_empty(self):
        cls = ClaimsList()
        self.assertEqual(0, cls.size())

    def test_add(self):
        cls = ClaimsList()
        cls.add("#1 @ 1,3: 4x4")
        self.assertEqual(1, cls.size())
        cls.add("#123 @ 3,2: 5x4")
        self.assertEqual(2, cls.size())
        cls.add("#124 @ 3,2: 5x5")
        self.assertEqual(3, cls.size())
        cls.add("#125 @ 3,2: 4x4")
        self.assertEqual(4, cls.size())

    def test_parse_claim(self):
        self.assertEqual((1, 1, 3, 4, 4), ClaimsList.parse_claim("#1 @ 1,3: 4x4"))
        self.assertEqual((123, 3, 2, 5, 4), ClaimsList.parse_claim("#123 @ 3,2: 5x4"))

    def test_count_overlaps(self):
        cls = ClaimsList()
        cls.add("#1 @ 1,3: 4x4")
        cls.add("#2 @ 3,1: 4x4")
        cls.add("#3 @ 5,5: 2x2")

        self.assertEqual(4, cls.count_overlaps())

        with open(data_file(2018, "day_03_mine.txt")) as f:
            claims_list = f.readlines()
            claims_mine = ClaimsList()
            for claim_string in claims_list:
                claims_mine.add(claim_string.strip())
            self.assertEqual(110389, claims_mine.count_overlaps())

    def test_find_non_overlapped(self):
        cls = ClaimsList()
        cls.add("#1 @ 1,3: 4x4")
        cls.add("#2 @ 3,1: 4x4")
        cls.add("#3 @ 5,5: 2x2")

        self.assertEqual([3], cls.find_non_overlapped())

        with open(data_file(2018, "day_03_mine.txt")) as f:
            claims_list = f.readlines()
            claims_mine = ClaimsList()
            for claim_string in claims_list:
                claims_mine.add(claim_string.strip())
            self.assertEqual([552], claims_mine.find_non_overlapped())


class ClaimsList:
    def __init__(self):
        self._claims = {}
        self._claim_grid = collections.defaultdict(list)

    def add(self, claim_string):
        claim_id, left, top, width, height = self.parse_claim(claim_string)
        right = left + width
        bottom = top + height
        self._claims[claim_id] = (left, right, top, bottom)
        for x in range(left, right):
            for y in range(top, bottom):
                self._claim_grid[x, y].append(claim_id)

    def size(self):
        return len(self._claims)

    def count_overlaps(self):
        count = 0
        for claims in self._claim_grid.values():
            if len(claims) > 1:
                count += 1
        return count

    def find_non_overlapped(self):
        claim_ids = set(self._claims.keys())
        for claims in self._claim_grid.values():
            if len(claims) > 1:
                claim_ids.difference_update(claims)
        return list(claim_ids)

    @staticmethod
    def parse_claim(claim_string):
        claim_re = re.compile(
            "^#(?P<claim_id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$"
        )
        match = claim_re.match(claim_string)
        if match is None:
            return None

        def int_for(group):
            return int(match.group(group))

        return (
            int_for("claim_id"),
            int_for("left"),
            int_for("top"),
            int_for("width"),
            int_for("height"),
        )
