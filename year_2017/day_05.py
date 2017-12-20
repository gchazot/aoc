import unittest
from aoc_utils import data_file


def alteration_one(maze, index):
    maze[index] += 1
    return maze[index] - 1


def alteration_two(maze, index):
    if maze[index] >= 3:
        maze[index] -= 1
        return maze[index] + 1
    else:
        maze[index] += 1
        return maze[index] - 1


def solve_maze(maze, alteration):
    index = 0
    steps = 0
    exit_index = len(maze)
    while index < exit_index:
        index += alteration(maze, index)
        steps += 1
    return steps


def solve_maze_file(filename, alteration):
    with open(filename) as f:
        maze_data = list(map(int, f.readlines()))
        return solve_maze(maze_data, alteration)


class TestSolveMaze(unittest.TestCase):
    example_data_file = data_file(2017, "day_05_1_example.txt")
    my_data_file = data_file(2017, "day_05_1_mine.txt")

    def test_1_example(self):
        self.assertEqual(5, solve_maze_file(self.example_data_file, alteration_one))

    def test_1_mine(self):
        self.assertEqual(374269, solve_maze_file(self.my_data_file, alteration_one))

    def test_2_example(self):
        self.assertEqual(10, solve_maze_file(self.example_data_file, alteration_two))

    @unittest.skip("Takes too long")
    def test_2_mine(self):
        self.assertEqual(27720699, solve_maze_file(self.my_data_file, alteration_two))