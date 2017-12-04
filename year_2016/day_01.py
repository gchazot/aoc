from unittest import TestCase
import functools


def parse(text):
    instructions = map(lambda s: s.strip(), text.split(','))
    return map(lambda i: (i[0], i[1:]), instructions)


def rotate_left(mask):
    return [-mask[1], mask[0]]


def rotate_right(mask):
    return [mask[1], -mask[0]]


def progress(direction, blocks):
    return map(lambda x: x * blocks, direction)


def advance(totals, step):
    return map(lambda x, y: x + y, totals, step)


def shortest_distance(instructions):
    position = [0, 0]
    direction = [0, 1]
    orders = {"R": rotate_right, "L": rotate_left}

    for rotation, blocks in parse(instructions):
        direction = orders[rotation](direction)
        step = progress(direction, int(blocks))
        position = advance(position, step)

    return functools.reduce(lambda x, y: abs(x) + abs(y), position)


my_instructions = ("R5, R4, R2, L3, R1, R1, L4, L5, R3, L1, L1, R4, L2, R1, R4, R4, L2, L2, R4, L4, R1, R3, L3, L1, " +
                   "L2, R1, R5, L5, L1, L1, R3, R5, L1, R4, L5, R5, R1, L185, R4, L1, R51, R3, L2, R78, R1, L4, " +
                   "R188, R1, L5, R5, R2, R3, L5, R3, R4, L1, R2, R2, L4, L4, L5, R5, R4, L4, R2, L5, R2, L1, L4, " +
                   "R4, L4, R2, L3, L4, R2, L3, R3, R2, L2, L3, R4, R3, R1, L4, L2, L5, R4, R4, L1, R1, L5, L1, R3, " +
                   "R1, L2, R1, R1, R3, L4, L1, L3, R2, R4, R2, L2, R1, L5, R3, L3, R3, L1, R4, L3, L3, R4, L2, L1, " +
                   "L3, R2, R3, L2, L1, R4, L3, L5, L2, L4, R1, L4, L4, R3, R5, L4, L1, L1, R4, L2, R5, R1, R1, R2, " +
                   "R1, R5, L1, L3, L5, R2")


class TestShortestDistance(TestCase):
    def validate(self, expected, instructions):
        self.assertEqual(expected, shortest_distance(instructions))

    def test_1(self):
        self.validate(5, "R2, L3")

    def test_2(self):
        self.validate(2, "R2, R2, R2")

    def test_3(self):
        self.validate(12, "R5, L5, R5, R3")

    def test_mine(self):
        self.validate(231, my_instructions)


def shortest_distance_visited_twice(instructions):
    position = [0, 0]
    direction = [0, 1]
    orders = {"R": rotate_right, "L": rotate_left}
    visited = [position]

    def distance(to_position):
        return functools.reduce(lambda x, y: abs(x) + abs(y), to_position)

    for rotation, blocks in parse(instructions):
        direction = orders[rotation](direction)
        for _ in range(int(blocks)):
            position = advance(position, direction)
            if position in visited:
                return distance(position)
            visited.append(position)

    return distance(position)


class TestFirstVisitedTwice(TestCase):
    def validate(self, expected, instructions):
        self.assertEqual(expected, shortest_distance_visited_twice(instructions))

    def test_1(self):
        self.validate(4, "R8, R4, R4, R8")

    def test_mine(self):
        self.validate(147, my_instructions)
