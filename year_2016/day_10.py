from functools import reduce
import itertools
from operator import mul
import re
import unittest
from aoc_utils.data import data_lines


class TestZoomingBots(unittest.TestCase):
    def test_parse_example(self):
        lines = data_lines(2016, "day_10_example.txt")
        bots = ZoomingBots(lines)

        self.assertEqual(9, len(bots.nodes))
        self.assertSetEqual(
            frozenset(itertools.product(('input', 'bot', 'output'), (0, 1, 2))),
            frozenset(bots.nodes.keys()),
        )
        for (type, id), data in bots.nodes.items():
            if type == 'input':
                self.assertEqual(1, len(data["values"]))
                self.assertEqual(1, len(data["outs"]))
            elif type == 'bot':
                self.assertEqual(0, len(data["values"]))
                self.assertEqual(2, len(data["outs"]))
            elif type == 'output':
                self.assertEqual(0, len(data["values"]))
                self.assertEqual(0, len(data["outs"]))

    def test_execute_example(self):
        lines = data_lines(2016, "day_10_example.txt")
        bots = ZoomingBots(lines)

        bots.execute()

        for (type, id), data in bots.nodes.items():
            if type == 'output':
                self.assertEqual(1, len(data['values']))
            elif type == 'bot':
                self.assertEqual(2, len(data['values']))

        self.assertEqual(('bot', 2), bots.find_comparator(2, 5))

    def test_execute_mine(self):
        lines = data_lines(2016, "day_10_mine.txt")
        bots = ZoomingBots(lines)

        bots.execute()

        self.assertEqual(('bot', 147), bots.find_comparator(17, 61))

        outputs = [bots.get_output(i) for i in range(3)]
        self.assertEqual(55637, reduce(mul, outputs))


value_pattern = re.compile(r"^value (\d+) goes to bot (\d+)$")
transfer_pattern = re.compile(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")


class ZoomingBots:
    def __init__(self, lines=None):
        self.nodes = {}

        if lines is not None:
            self.parse(lines)

    def parse(self, lines):
        next_input_id = len([None for node in self.nodes.keys() if node[0] == 'input'])
        for line in lines:
            value_match = value_pattern.match(line)
            if value_match is not None:
                value = int(value_match.group(1))
                destination = "bot", int(value_match.group(2))
                self._set_node(("input", next_input_id), [value], [destination])
                next_input_id += 1
                continue

            transfer_match = transfer_pattern.match(line)
            if transfer_match is not None:
                origin = int(transfer_match.group(1))
                low = transfer_match.group(2), int(transfer_match.group(3))
                high = transfer_match.group(4), int(transfer_match.group(5))
                self._set_node(("bot", origin), [], [low, high])
                for output in (low, high):
                    if output[0] == 'output':
                        self._set_node(output, [], [])
                continue

            raise RuntimeError("Unparsed: '{}'".format(line))

    def _set_node(self, identity, values, outs):
        self.nodes[identity] = {
            "values": values,
            "outs": outs,
        }

    def execute(self):
        active = {identity for identity in self.nodes.keys() if identity[0] == 'input'}

        while active:
            temp = active.copy()
            active = set()
            for current in temp:
                data = self.nodes[current]
                values = data['values']
                outs = data['outs']
                assert len(values) == len(outs)

                pairs = zip(outs, sorted(values))

                for out, value in pairs:
                    output = self.nodes[out]
                    output['values'].append(value)
                    if len(output['values']) == len(output['outs']):
                        active.add(out)

    def find_comparator(self, *values):
        for identity, data in self.nodes.items():
            if frozenset(data['values']) == frozenset(values):
                return identity

    def get_output(self, id):
        return self.nodes[('output', id)]['values'][0]