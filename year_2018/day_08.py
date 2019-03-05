from aoc_utils.data import data_text
import unittest


class TestKeyNode(unittest.TestCase):
    def test_example(self):
        tree_string = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
        tree_data = [int(num) for num in reversed(tree_string.split())]
        tree = KeyNode(tree_data)

        self.assertEqual(2, len(tree.children))
        self.assertEqual(3, len(tree.metadata))
        self.assertEqual(138, tree.sum_metadata())
        self.assertEqual(66, tree.value())

    def test_mine(self):
        tree_string = data_text(2018, 'day_08_mine.txt')
        tree_data = [int(num) for num in reversed(tree_string.split())]
        tree = KeyNode(tree_data)

        self.assertEqual(43351, tree.sum_metadata())
        self.assertEqual(21502, tree.value())


class KeyNode:
    def __init__(self, licence_file):
        num_children = licence_file.pop()
        num_metadata = licence_file.pop()
        self.children = [KeyNode(licence_file) for _ in range(num_children)]
        self.metadata = [licence_file.pop() for _ in range(num_metadata)]

    def sum_metadata(self):
        return sum(self.metadata) + sum(child.sum_metadata() for child in self.children)

    def value(self):
        num_children = len(self.children)
        if num_children == 0:
            return sum(self.metadata)
        else:
            return sum(self.child_value(i) for i in self.metadata if 0 < i <= num_children)

    def child_value(self, child_num):
        child_index = child_num - 1
        return self.children[child_index].value()
