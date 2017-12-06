from unittest import TestCase

import math

from collections import defaultdict


def shortest_route_1_iterative(location):
    if location == 1:
        return 0
    i = 0
    n = 1
    while n + 8 * i < location:
        n += 8 * i
        i += 1
    return i + (location - n + i) % (2 * i)


def shortest_route_1_direct(location):
    if location == 1:
        return 0
    i = round(math.sqrt(location - 1) / 2)
    return i + (location - i * (4 * i - 3) - 1) % (2 * i)


class Walker:
    HORIZONTAL = 0
    VERTICAL = 1

    def __init__(self):
        self.position = [1, 0]
        self.direction = self.VERTICAL
        self.sense = 1
        self.loop_number = 1
        self.loop_steps = 1

    def get_position(self):
        return self.format_position(self.position)

    @staticmethod
    def format_position(position):
        return "{0},{1}".format(*position)

    def walk(self):
        self.step_forward()
        if self.loop_steps == 8 * self.loop_number:
            self.loop_number += 1
            self.loop_steps = 0
        if abs(self.position[self.direction]) == self.loop_number:
            self.turn()

    def turn(self):
        if self.direction == self.VERTICAL:
            self.sense = -self.sense
        self.direction = 1 - self.direction

    def step_forward(self):
        self.position[self.direction] += self.sense
        self.loop_steps += 1

    def surroundings(self):
        return map(self.format_position,
                   [[self.position[0] - 1, self.position[1] + 1],
                    [self.position[0] - 1, self.position[1]],
                    [self.position[0] - 1, self.position[1] - 1],
                    [self.position[0], self.position[1] - 1],
                    [self.position[0], self.position[1] + 1],
                    [self.position[0] + 1, self.position[1] + 1],
                    [self.position[0] + 1, self.position[1]],
                    [self.position[0] + 1, self.position[1] - 1]
                    ]
                   )


def next_higher_value(location):
    walker = Walker()
    grid = defaultdict(lambda: 0)
    grid[Walker.format_position([0, 0])] = 1
    while True:
        position = walker.get_position()
        value = sum((grid[neighbour] for neighbour in walker.surroundings()))
        grid[position] = value
        if value > location:
            return value
        walker.walk()


class TestShortestRoute(TestCase):
    def test_1_example1(self):
        self.assertEqual(0, shortest_route_1_direct(1))

    def test_1_example2(self):
        self.assertEqual(3, shortest_route_1_direct(12))

    def test_1_example3(self):
        self.assertEqual(2, shortest_route_1_direct(23))

    def test_1_example4(self):
        self.assertEqual(31, shortest_route_1_direct(1024))

    def test_1_mine(self):
        self.assertEqual(326, shortest_route_1_direct(361527))

    def test_next_higher_value(self):
        def check(location, expected):
            self.assertEqual(expected, next_higher_value(location))

        check(2, 4)
        check(4, 5)
        check(5, 10)
        check(11, 23)
        check(23, 25)
        check(24, 25)
        check(25, 26)
        check(26, 54)
        check(27, 54)
        check(45, 54)
        check(53, 54)
        check(54, 57)
        check(142, 147)

    def test_next_higher_value_mine(self):
        self.assertEqual(363010, next_higher_value(361527))

