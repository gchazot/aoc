from unittest import TestCase
from aoc_utils import data_text


class StreamParser:
    def __init__(self, stream, length, nesting=0, start=0):
        self.stream = stream
        self.length = length
        self.position = start
        self.nesting = nesting
        self.score = nesting
        self.in_rubbish = False
        self.rubbish_count = 0

    def parse(self):
        while self.position < self.length:
            char = self.stream[self.position]
            if char == "!":
                self.position += 1
            elif self.in_rubbish:
                if char == ">":
                    self.in_rubbish = False
                else:
                    self.rubbish_count += 1
            elif char == "<":
                self.in_rubbish = True
            else:
                if char == "{":
                    sub_parser = StreamParser(self.stream, self.length, self.nesting + 1, self.position + 1)
                    sub_parser.parse()
                    self.score += sub_parser.score
                    self.rubbish_count += sub_parser.rubbish_count
                    self.position = sub_parser.position
                elif char == "}":
                    return
            self.position += 1
        return


def score_stream(stream):
    parser = StreamParser(stream, len(stream))
    parser.parse()
    return parser.score, parser.rubbish_count


class TestStreamParse(TestCase):
    def test_no_group_gets_score_0(self):
        self.assertEqual(0, score_stream("")[0])

    def test_simple_group_gets_score_1(self):
        self.assertEqual(1, score_stream("{}")[0])

    def test_nested_group_gets_score_plus_1(self):
        self.assertEqual(2 + 1, score_stream("{{}}")[0])
        self.assertEqual(2 + 2 + 1, score_stream("{{},{}}")[0])
        self.assertEqual(3 + 2 + 2 + 1, score_stream("{{{}},{}}")[0])

    def test_ignores_escaped_chars(self):
        self.assertEqual(0, score_stream("!{}")[0])
        self.assertEqual(1, score_stream("!{{}}")[0])
        self.assertEqual(1, score_stream("{!{}}")[0])

    def test_ignores_rubbish(self):
        self.assertEqual(1, score_stream("{<abc>}")[0])
        self.assertEqual(1, score_stream("{<{}>}")[0])
        self.assertEqual(3, score_stream("{<{{}<>{}>}")[0])
        self.assertEqual(1, score_stream("{<a>,<a>,<a>,<a>}")[0])
        self.assertEqual(9, score_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}")[0])
        self.assertEqual(9, score_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}")[0])
        self.assertEqual(3, score_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}")[0])

    def test_counts_rubbish(self):
        self.assertEqual(3, score_stream("{<abc>}")[1])
        self.assertEqual(2, score_stream("{<{}>}")[1])
        self.assertEqual(4, score_stream("{<{{}<>{}>}")[1])
        self.assertEqual(4, score_stream("{<a>,<a>,<a>,<a>}")[1])
        self.assertEqual(8, score_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}")[1])
        self.assertEqual(0, score_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}")[1])
        self.assertEqual(17, score_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}")[1])

    def test_parse_stream_mine(self):
        stream = data_text(2017, "day_09_mine.txt")
        self.assertEqual((11898, 5601), score_stream(stream))
