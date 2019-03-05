from unittest import TestCase
from aoc_utils.data import data_lines


def read_network(filename):
    network = {}
    for line in data_lines(2017, filename):
        program_id_str, contacts_str = line.split(" <-> ")
        program_id = int(program_id_str)
        contact_list_str = contacts_str.split(", ")
        contacts = map(int, contact_list_str)
        network[program_id] = list(contacts)
    return network


class TuyauNet:
    def __init__(self, filename):
        self.nodes = read_network(filename)

    class Visitor:
        def __init__(self, network):
            self.nodes = network.nodes
            self.distance = [None for _ in range(len(network.nodes))]

        def visit(self, node, distance):
            self.distance[node] = distance
            for neighbour in self.nodes[node]:
                if self.distance[neighbour] is None:
                    self.visit(neighbour, distance + 1)

        def next_unvisited(self):
            return next(i for i in range(len(self.distance)) if self.distance[i] is None)

    def visit(self, start_point):
        visitor = TuyauNet.Visitor(self)
        visitor.visit(start_point, 0)
        return visitor.distance

    def count_groups(self):
        groups = 0
        visitor = TuyauNet.Visitor(self)
        try:
            while True:
                start_point = visitor.next_unvisited()
                visitor.visit(start_point, 0)
                groups += 1
        except StopIteration:
            pass
        return groups


class TestTuyauCom(TestCase):
    def test_read_network(self):
        network = read_network("day_12_example.txt")
        self.assertEqual(7, len(network))
        self.assertEqual(1, len(network[0]))
        self.assertEqual(3, len(network[2]))
        self.assertEqual(2, len(network[6]))

    def test_visit_example_nodes(self):
        network = TuyauNet("day_12_example.txt")
        visited = network.visit(0)
        self.assertEqual(7, len(visited))
        self.assertEqual(6, len([None for node in visited if node is not None]))

    def test_visit_my_nodes(self):
        network = TuyauNet("day_12_mine.txt")
        visited = network.visit(0)
        self.assertEqual(2000, len(visited))
        self.assertEqual(288, len([None for node in visited if node is not None]))

    def test_count_groups_example(self):
        network = TuyauNet("day_12_example.txt")
        self.assertEqual(2, network.count_groups())

    def test_count_groups_mine(self):
        network = TuyauNet("day_12_mine.txt")
        self.assertEqual(211, network.count_groups())
