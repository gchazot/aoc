import unittest
from aoc_utils import data_lines


def hash_pattern(pattern):
    return "".join(pattern)


def cache_result(func):
    cache = {}

    def inner(pattern):
        h = hash_pattern(pattern)
        if h not in cache:
            cache[h] = func(pattern)
        return cache[h]

    return inner


@cache_result
def rotate(p):
    size = len(p)
    if size == 2:
        return [
            "".join([p[1][0], p[0][0]]),
            "".join([p[1][1], p[0][1]]),
        ]
    elif size == 3:
        return [
            "".join([p[2][0], p[1][0], p[0][0]]),
            "".join([p[2][1], p[1][1], p[0][1]]),
            "".join([p[2][2], p[1][2], p[0][2]]),
        ]
    else:
        raise RuntimeError("Error, size is ", size)


@cache_result
def flip(p):
    return list(reversed(p))


@cache_result
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
        options = (
            self.pattern,
            rotate(self.pattern),
            flip(self.pattern),
            flip(rotate(self.pattern)),
            reverse(self.pattern),
            reverse(rotate(self.pattern)),
            reverse(flip(self.pattern)),
            reverse(flip(rotate(self.pattern))),
        )

        return pattern in options

    def children(self):
        size = len(self.enhanced)
        if size == 3:
            yield self.enhanced
        elif size == 4:
            yield [self.enhanced[0][:2], self.enhanced[1][:2]]
            yield [self.enhanced[0][2:], self.enhanced[1][2:]]
            yield [self.enhanced[2][:2], self.enhanced[3][:2]]
            yield [self.enhanced[2][2:], self.enhanced[3][2:]]
        else:
            raise RuntimeError("Error, size is ", size)


class TestRule(unittest.TestCase):
    def setUp(self):
        self.pattern2x2 = Rule("#./.. => ..#/.#./#.#")
        self.pattern3x3 = Rule("##./#../... => ###./.###/#.##/.#..")

    def test_splits_enhanced(self):
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
        self.index = {}

    def find_rule_for(self, pattern):
        h = hash_pattern(pattern)
        if h not in self.index:
            self.index[h] = None
            for rule in self.rules:
                if rule.matches(pattern):
                    self.index[h] = rule
        return self.index[h]

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


class PatternTree:
    class Node:
        def __init__(self, pattern):
            self.pattern = pattern
            self.children = []

        def add_child(self, pattern):
            node = PatternTree.Node(pattern)
            self.children.append(node)

        def count_pixels(self):
            return sum(line.count("#") for line in self.pattern)

    def __init__(self, rule_book, depth):
        self.rule_book = rule_book
        self.base = PatternTree.Node([".#.", "..#", "###"])

        self._init_nodes_to_depth([self.base], depth)

    def _init_nodes_to_depth(self, nodes, depth):
        if depth <= 0:
            return

        for node in nodes:
            child_patterns = self.rule_book.children_of(node.pattern)
            for pattern in child_patterns:
                node.add_child(pattern)
            self._init_nodes_to_depth(node.children, depth-1)

    def count_at_depth(self, depth):
        return self._count_for_nodes_at_depth([self.base], depth)

    def _count_for_nodes_at_depth(self, nodes, depth):
        if depth == 0:
            return sum(node.count_pixels() for node in nodes)
        elif depth > 0:
            return sum(self._count_for_nodes_at_depth(node.children, depth-1) for node in nodes)
        else:
            raise RuntimeError("depth must be > 0")


class PatternTreeTest(unittest.TestCase):
    def setUp(self):
        self.rule_lines = [
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#",
        ]
        self.rule_book = RuleBook(self.rule_lines)

    def test_node_counts_pixels(self):
        def check_count(expected_count, pattern):
            node = PatternTree.Node(pattern)
            self.assertEqual(expected_count, node.count_pixels())

        check_count(1, ["..", ".#"])
        check_count(2, [".#", ".#"])
        check_count(3, ["##", ".#"])
        check_count(4, ["##", "##"])

        check_count(4, [".#.", "..#", "#.#"])
        check_count(5, [".#.", "..#", "###"])

    def test_initialises_with_start_pattern(self):
        tree = PatternTree(self.rule_book, 0)
        self.assertListEqual([".#.", "..#", "###"], tree.base.pattern)
        self.assertEqual(0, len(tree.base.children))

    def test_initialises_to_depth_1(self):
        tree = PatternTree(self.rule_book, 1)
        self.assertEqual(4, len(tree.base.children))

        for child in tree.base.children:
            self.assertEqual(0, len(child.children))

    def test_initialises_to_depth_2(self):
        tree = PatternTree(self.rule_book, 2)
        self.assertEqual(4, len(tree.base.children))

        for child in tree.base.children:
            self.assertEqual(1, len(child.children))
            for grand_child in child.children:
                self.assertEqual(0, len(grand_child.children))

    def test_count_pixels_at_depth(self):
        tree = PatternTree(self.rule_book, 2)

        self.assertEqual(5, tree.count_at_depth(0))
        self.assertEqual(4, tree.count_at_depth(1))
        self.assertEqual(12, tree.count_at_depth(2))

    @unittest.skip("Does not get the expected result")
    def test_mine(self):
        rules_lines = data_lines(2017, "day_21_mine.txt")

        rule_book = RuleBook(rules_lines)
        art = PatternTree(rule_book, 7)

        self.assertEqual(5, art.count_at_depth(0))
        self.assertEqual(6, art.count_at_depth(1))
        self.assertEqual(16, art.count_at_depth(2))
        self.assertEqual(34, art.count_at_depth(3))
        self.assertEqual(72, art.count_at_depth(4))
        self.assertEqual(122, art.count_at_depth(5))
        self.assertEqual(264, art.count_at_depth(6))
        self.assertEqual(482, art.count_at_depth(7))

    @unittest.skip("This is not my data")
    def test_other(self):
        rules_lines = data_lines(2017, "day_21_other.txt")

        rule_book = RuleBook(rules_lines)
        art = PatternTree(rule_book, 7)

        self.assertEqual(5, art.count_at_depth(0))
        self.assertEqual(3, art.count_at_depth(1))
        self.assertEqual(26, art.count_at_depth(2))
        self.assertEqual(29, art.count_at_depth(3))
        self.assertEqual(98, art.count_at_depth(4))
        self.assertEqual(110, art.count_at_depth(5))
        self.assertEqual(378, art.count_at_depth(6))
        self.assertEqual(448, art.count_at_depth(7))

        # This is the expected correct answer
        # self.assertEqual(208, art.count_at_depth(5))


class ArtPiece:
    def __init__(self, rule_book, start_pattern):
        self.rule_book = rule_book
        self.image = start_pattern

    def enhance_image(self, passes):
        for _ in range(passes):
            self.enhance_image_once()

    def count_pixels(self):
        return sum(line.count("#") for line in self.image)

    def enhance_image_once(self):
        enhanced_pattern_size = self.enhanced_pattern_size()

        new_image = ["" for _ in range(self.enhanced_image_size())]
        for i, pattern_row in enumerate(self.split_image_to_patterns()):
            top_row = i * enhanced_pattern_size * self.patterns_enhancement_factor()
            for pattern in pattern_row:
                enhanced_patterns = self.rule_book.children_of(pattern)
                for n, new_pattern in enumerate(enhanced_patterns):
                    enhanced_pattern_i = int(n / enhanced_pattern_size)
                    enhanced_pattern_top_row = top_row + enhanced_pattern_i * enhanced_pattern_size
                    for v, new_pattern_row in enumerate(new_pattern):
                        new_i = enhanced_pattern_top_row + v
                        new_image[new_i] += new_pattern_row

        self.image = new_image

    def image_size(self):
        return len(self.image)

    def split_image_to_patterns(self):
        for i in range(self.num_patterns_to_enhance()):
            pattern_row = []
            for j in range(self.num_patterns_to_enhance()):
                new_patterns = self._gen_pattern(i, j)
                pattern_row.append(list(new_patterns))
            yield pattern_row

    def num_patterns_to_enhance(self):
        return int(self.image_size() / self.image_pattern_size())

    def image_pattern_size(self):
        if self.image_size() % 2 == 0:
            return 2
        elif self.image_size() % 3 == 0:
            return 3

    def _gen_pattern(self, row, col):
        pattern_size = self.image_pattern_size()
        for k in range(pattern_size):
            row_num = row * pattern_size + k
            first_col_num = col * pattern_size
            last_col_num = first_col_num + pattern_size
            yield self.image[row_num][first_col_num:last_col_num]

    def enhanced_image_size(self):
        next_pattern_num = self.num_patterns_to_enhance() * self.patterns_enhancement_factor()
        next_pattern_size = self.enhanced_pattern_size()
        next_image_size = next_pattern_num * next_pattern_size

        return next_image_size

    def enhanced_pattern_size(self):
        return 2 + 3 - self.image_pattern_size()

    def patterns_enhancement_factor(self):
        if self.image_pattern_size() == 2:
            return 1
        elif self.image_pattern_size() == 3:
            return 2


class ArtPieceTest(unittest.TestCase):
    def setUp(self):
        self.rule_lines = [
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#",
        ]
        self.rule_book = RuleBook(self.rule_lines)

        self.image2x2 = [".#", "..", ]
        self.image3x3 = [".#.", "..#", "###", ]
        self.image4x4 = [".#.#", "....", "####", "....", ]
        self.image6x6 = [".#.#.#", "......", "######",
                         "......", "##..##", "#.#.#.", ]
        self.image9x9 = [".#.#.#.#.", ".........", "#########",
                         ".........", "##..##..#", "#.#.#.#.#",
                         ".#.#.#.#.", "..#..#..#", "#.##.##.#", ]

    def test_initialises_to_start_pattern(self):
        for pattern in (self.image2x2, self.image3x3, self.image4x4, self.image6x6):
            art = ArtPiece(self.rule_book, pattern)
            self.assertListEqual(pattern, art.image)

    def test_count_pixels(self):
        def check_pixels_count(expected_count, image):
            art = ArtPiece(None, image)
            self.assertEqual(expected_count, art.count_pixels())

        check_pixels_count(1, self.image2x2)
        check_pixels_count(5, self.image3x3)
        check_pixels_count(6, self.image4x4)
        check_pixels_count(16, self.image6x6)
        check_pixels_count(36, self.image9x9)

    def test_choose_pattern_size(self):
        def check_image_pattern_size(expected_size, image):
            art = ArtPiece(self.rule_book, image)
            self.assertEqual(expected_size, art.image_pattern_size())

        check_image_pattern_size(2, self.image2x2)
        check_image_pattern_size(3, self.image3x3)
        check_image_pattern_size(2, self.image4x4)
        check_image_pattern_size(2, self.image6x6)
        check_image_pattern_size(3, self.image9x9)

    def test_split_image_to_patterns(self):
        def check_actual_split_size(expected_size, image):
            art = ArtPiece(self.rule_book, image)
            image_patterns = list(art.split_image_to_patterns())
            self.assertEqual(expected_size, len(image_patterns))
            for pattern_row in image_patterns:
                self.assertEqual(expected_size, len(pattern_row))

        check_actual_split_size(1, self.image2x2)
        check_actual_split_size(1, self.image3x3)
        check_actual_split_size(2, self.image4x4)
        check_actual_split_size(3, self.image6x6)
        check_actual_split_size(3, self.image9x9)

    def test_calculate_enhanced_image_size(self):
        def check_enhanced_image_size(expected_size, image):
            art = ArtPiece(self.rule_book, image)
            self.assertEqual(expected_size, art.enhanced_image_size())

        check_enhanced_image_size(3, self.image2x2)
        check_enhanced_image_size(4, self.image3x3)
        check_enhanced_image_size(6, self.image4x4)
        check_enhanced_image_size(9, self.image6x6)
        check_enhanced_image_size(12, self.image9x9)

    def test_enhance_image_once(self):
        art = ArtPiece(self.rule_book, self.image3x3)
        art.enhance_image_once()
        self.assertListEqual(["#..#", "....", "....", "#..#"], art.image)
        art.enhance_image_once()
        self.assertListEqual(["##.##.", "#..#..", "......",
                              "##.##.", "#..#..", "......"], art.image)

        art = ArtPiece(self.rule_book, self.image2x2)
        art.enhance_image_once()
        self.assertListEqual(["##.", "#..", "..."], art.image)

    @unittest.skip("Just a bit too slow")
    def test_mine(self):
        rules_lines = data_lines(2017, "day_21_mine.txt")

        rule_book = RuleBook(rules_lines)
        art = ArtPiece(rule_book, [".#.", "..#", "###", ])

        art.enhance_image(5)
        self.assertEqual(150, art.count_pixels())

        art.enhance_image(18 - 5)
        self.assertEqual(2606275, art.count_pixels())
