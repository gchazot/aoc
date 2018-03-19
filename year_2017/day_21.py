import unittest


def rotate(p):
    if len(p) == 2:
        return [
            "".join([p[1][0], p[0][0]]),
            "".join([p[1][1], p[0][1]]),
        ]
    else:
        return [
            "".join([p[1][0], p[0][0], p[0][1]]),
            "".join([p[2][0], p[1][1], p[0][2]]),
            "".join([p[2][1], p[2][2], p[1][2]]),
        ]


def flip(p):
    return list(reversed(p))


def reverse(p):
    return ["".join(reversed(line)) for line in reversed(p)]


class Rule:
    def __init__(self, line):
        pattern, enhanced = line.strip().split(" => ")
        self.pattern = pattern.split("/")
        self.enhanced = enhanced.split("/")

    def matches(self, pattern):
        return pattern in (
            self.pattern,
            rotate(self.pattern),
            flip(self.pattern),
            flip(rotate(self.pattern)),
            reverse(self.pattern),
            reverse(rotate(self.pattern)),
            reverse(flip(self.pattern)),
            reverse(flip(rotate(self.pattern))),
        )


class TestRule(unittest.TestCase):
    def setUp(self):
        self.pattern2x2 = Rule("#./.. => ..#/.#./#.#")
        self.pattern3x3 = Rule("##./#../... => ###./.###/#.##/.#..")

    def test_init(self):
        self.assertEqual(["..#", ".#.", "#.#"], self.pattern2x2.enhanced)
        self.assertEqual(["###.", ".###", "#.##", ".#.."], self.pattern3x3.enhanced)

    def test_ignores_end_of_line_in_input(self):
        with_end_of_line = Rule("#./.. => ..#/.#./#.#\n")
        self.assertEqual(["..#", ".#.", "#.#"], with_end_of_line.enhanced)

    def test_matches_same(self):
        self.assertTrue(self.pattern2x2.matches(["#.", ".."]))
        self.assertTrue(self.pattern3x3.matches(["##.", "#..", "..."]))

    def test_matches_rotated(self):
        self.assertTrue(self.pattern2x2.matches([".#", ".."]))
        self.assertTrue(self.pattern3x3.matches(["###", "...", "..."]))

    def test_matches_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["..", "#."]))
        self.assertTrue(self.pattern3x3.matches(["...", "#..", "##."]))

    def test_matches_rotated_and_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["..", ".#"]))
        self.assertTrue(self.pattern3x3.matches(["...", "...", "###"]))

    def test_matches_reversed(self):
        self.assertTrue(self.pattern2x2.matches(["..", ".#"]))
        self.assertTrue(self.pattern3x3.matches(["...", "..#", ".##"]))

    def test_matches_reversed_rotated(self):
        self.assertTrue(self.pattern2x2.matches(["..", "#."]))
        self.assertTrue(self.pattern3x3.matches(["...", "...", "###"]))

    def test_matches_reversed_flipped(self):
        self.assertTrue(self.pattern2x2.matches([".#", ".."]))
        self.assertTrue(self.pattern3x3.matches([".##", "..#", "..."]))

    def test_matches_reversed_rotated_and_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["#.", ".."]))
        self.assertTrue(self.pattern3x3.matches(["...", "...", "###"]))

    def test_rotate(self):
        self.assertListEqual([".#", ".."], rotate(self.pattern2x2.pattern))
        self.assertListEqual(["###", "...", "..."], rotate(self.pattern3x3.pattern))

    def test_flip(self):
        self.assertListEqual(["..", "#."], flip(self.pattern2x2.pattern))
        self.assertListEqual(["...", "#..", "##."], flip(self.pattern3x3.pattern))

    def test_reverse(self):
        self.assertListEqual(["..", ".#"], reverse(self.pattern2x2.pattern))
        self.assertListEqual(["...", "..#", ".##"], reverse(self.pattern3x3.pattern))


class RuleBook:
    def __init__(self, lines):
        self.rules = [Rule(line) for line in lines]

    def find_rule_for(self, pattern):
        for rule in self.rules:
            if rule.matches(pattern):
                return rule
        return None


class TestRuleBook(unittest.TestCase):
    def setUp(self):
        self.rule_lines = [
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#",
        ]
        self.rule_book = RuleBook(self.rule_lines)

    def test_creates_with_lines(self):
        self.assertEqual(len(self.rule_lines), len(self.rule_book.rules))

    def test_find_rule_for(self):
        def enhance(pattern):
            rule = self.rule_book.find_rule_for(pattern)
            if rule is not None:
                return rule.enhanced

        self.assertListEqual(["##.", "#..", "..."], enhance(["..", ".#"]))
        self.assertListEqual(["#..#", "....", "....", "#..#"], enhance([".#.", "..#", "###"]))
        self.assertListEqual(["##.", "#..", "..."], enhance(["..", "#."]))
        self.assertListEqual(["#..#", "....", "....", "#..#"], enhance([".#.", "#..", "###"]))


class ArtPiece:
    def __init__(self, initial_pattern, rule_book):
        self.rule_book = rule_book
        initial_rule = self.rule_book.find_rule_for(initial_pattern)
        self.rules = [[initial_rule]]


class TestArtPiece(unittest.TestCase):
    def setUp(self):
        self.initial_pattern = [".#.", "..#", "###"]
        rule_lines = [
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#",
        ]
        self.rule_book = RuleBook(rule_lines)
        self.art_piece = ArtPiece(self.initial_pattern, self.rule_book)

    def test_initialises_with_rule(self):
        initial_rule = self.rule_book.find_rule_for(self.initial_pattern)
        self.assertListEqual([[initial_rule]], self.art_piece.rules)
