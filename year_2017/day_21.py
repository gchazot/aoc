import unittest


def rotate(p):
    if len(p) == 2:
        return [
            "".join([p[1][0], p[0][0]]),
            "".join([p[1][1], p[0][1]]),
        ]
    else:
        return [
            "".join([p[2][0], p[1][0], p[0][0]]),
            "".join([p[2][1], p[1][1], p[0][1]]),
            "".join([p[2][2], p[1][2], p[0][2]]),
        ]


def flip(p):
    return list(reversed(p))


def reverse(p):
    return ["".join(reversed(line)) for line in reversed(p)]


class TestAlterations(unittest.TestCase):
    def setUp(self):
        self.pattern2x2 = ["#.", ".."]
        self.pattern3x3 = ["##.", "#..", "..."]

    def test_rotate(self):
        self.assertListEqual([".#", ".."], rotate(self.pattern2x2))
        self.assertListEqual([".##", "..#", "..."], rotate(self.pattern3x3))
        self.assertListEqual(["#..", "#.#", "##."], rotate([".#.", "..#", "###"]))

    def test_flip(self):
        self.assertListEqual(["..", "#."], flip(self.pattern2x2))
        self.assertListEqual(["...", "#..", "##."], flip(self.pattern3x3))

    def test_reverse(self):
        self.assertListEqual(["..", ".#"], reverse(self.pattern2x2))
        self.assertListEqual(["...", "..#", ".##"], reverse(self.pattern3x3))


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

    def children(self):
        if len(self.enhanced) == 3:
            yield self.enhanced
        else:
            yield [self.enhanced[0][:2], self.enhanced[1][:2]]
            yield [self.enhanced[0][2:], self.enhanced[1][2:]]
            yield [self.enhanced[2][:2], self.enhanced[3][:2]]
            yield [self.enhanced[2][2:], self.enhanced[3][2:]]


class TestRule(unittest.TestCase):
    def setUp(self):
        self.pattern2x2 = Rule("#./.. => ..#/.#./#.#")
        self.pattern3x3 = Rule("##./#../... => ###./.###/#.##/.#..")

    def test_init(self):
        self.assertEqual(["..#", ".#.", "#.#"], self.pattern2x2.enhanced)
        self.assertEqual(["###.", ".###", "#.##", ".#.."], self.pattern3x3.enhanced)

    def test_returns_children_patterns(self):
        self.assertListEqual([["..#", ".#.", "#.#"]],
                             list(self.pattern2x2.children()))
        self.assertListEqual([["##", ".#"], ["#.", "##"], ["#.", ".#"], ["##", ".."]],
                             list(self.pattern3x3.children()))

    def test_ignores_end_of_line_in_input(self):
        with_end_of_line = Rule("#./.. => ..#/.#./#.#\n")
        self.assertEqual(["..#", ".#.", "#.#"], with_end_of_line.enhanced)

    def test_matches_same(self):
        self.assertTrue(self.pattern2x2.matches(["#.", ".."]))
        self.assertTrue(self.pattern3x3.matches(["##.", "#..", "..."]))

    def test_matches_rotated(self):
        self.assertTrue(self.pattern2x2.matches([".#", ".."]))
        self.assertTrue(self.pattern3x3.matches([".##", "..#", "..."]))

    def test_matches_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["..", "#."]))
        self.assertTrue(self.pattern3x3.matches(["...", "#..", "##."]))

    def test_matches_rotated_and_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["..", ".#"]))
        self.assertTrue(self.pattern3x3.matches(["...", "..#", ".##"]))

    def test_matches_reversed(self):
        self.assertTrue(self.pattern2x2.matches(["..", ".#"]))
        self.assertTrue(self.pattern3x3.matches(["...", "..#", ".##"]))

    def test_matches_reversed_rotated(self):
        self.assertTrue(self.pattern2x2.matches(["..", "#."]))
        self.assertTrue(self.pattern3x3.matches(["...", "#..", "##."]))

    def test_matches_reversed_flipped(self):
        self.assertTrue(self.pattern2x2.matches([".#", ".."]))
        self.assertTrue(self.pattern3x3.matches([".##", "..#", "..."]))

    def test_matches_reversed_rotated_and_flipped(self):
        self.assertTrue(self.pattern2x2.matches(["#.", ".."]))
        self.assertTrue(self.pattern3x3.matches(["##.", "#..", "..."]))


class RuleBook:
    def __init__(self, lines):
        self.rules = [Rule(line) for line in lines]

    def find_rule_for(self, pattern):
        for rule in self.rules:
            if rule.matches(pattern):
                return rule
        return None

    def children_of(self, pattern):
        rule = self.find_rule_for(pattern)
        return rule.children()


class TestRuleBook(unittest.TestCase):
    def setUp(self):
        self.rule_lines = [
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#",
            ".##/..#/... => #..#/.##./.##./#..#",
            "#./.# => #.#/#.#/.#.",
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

    def test_finds_rules_children(self):
        def check_children(pattern, expected_child_patterns):
            children = list(self.rule_book.children_of(pattern))
            self.assertEqual(len(expected_child_patterns), len(children))
            for i, child_pattern in enumerate(children):
                rule = self.rule_book.find_rule_for(child_pattern)
                expected_pattern = expected_child_patterns[i]
                self.assertTrue(rule.matches(expected_pattern))

        check_children(["..", ".#"], [["##.", "#..", "..."]])
        check_children([".#", ".."], [["##.", "#..", "..."]])

        check_children([".#.", "..#", "###"], [["#.", ".."] for _ in range(4)])
        check_children(["#..", "#.#", "##."], [["#.", ".."] for _ in range(4)])

        check_children([".##", "..#", "..."], [["#.", ".#"] for _ in range(4)])
        check_children(["##.", "#..", "..."], [["#.", ".#"] for _ in range(4)])


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
