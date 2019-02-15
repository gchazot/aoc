import unittest
from aoc_utils import data_lines


class TestCartNetwork(unittest.TestCase):
    def test_parse_initial_state(self):
        nw = CartNetwork()
        nw.parse_initial_state("day_13_example_1.txt")

    def test_get_ordered_carts(self):
        nw = CartNetwork()
        nw.parse_initial_state("day_13_example_1.txt")

        carts = nw._get_ordered_carts()
        self.assertEqual(2, len(carts))
        self.assertEqual((0, 2), carts[0])
        self.assertEqual((3, 9), carts[1])

    def test_tick(self):
        nw = CartNetwork()
        nw.parse_initial_state("day_13_example_1.txt")

        nw.tick()
        carts = nw._get_ordered_carts()
        self.assertEqual((0, 3), carts[0])
        self.assertEqual((4, 9), carts[1])

        nw.tick()
        carts = nw._get_ordered_carts()
        self.assertEqual((0, 4), carts[0])
        self.assertEqual((4, 10), carts[1])

        nw.tick()
        carts = nw._get_ordered_carts()
        self.assertEqual((1, 4), carts[0])
        self.assertEqual((4, 11), carts[1])

    def test_first_collision(self):
        example = CartNetwork()
        example.parse_initial_state("day_13_example_1.txt")

        self.assertEqual((7, 3), example.first_collision())

        mine = CartNetwork()
        mine.parse_initial_state("day_13_mine.txt")

        self.assertEqual((100, 21), mine.first_collision())

    def test_last_standing(self):
        example = CartNetwork()
        example.parse_initial_state("day_13_example_2.txt")

        self.assertEqual((6, 4), example.last_standing())

        mine = CartNetwork()
        mine.parse_initial_state("day_13_mine.txt")

        self.assertEqual((113, 109), mine.last_standing())


class Directions:
    up, down, left, right = list(range(4))


class CartNetwork:
    def __init__(self):
        self._bends = {}
        self._carts = {}
        self._intersections = set()
        self._collisions = []

    def parse_initial_state(self, filename):
        for i, line in enumerate(data_lines(2018, filename)):
            for j, c in enumerate(line.rstrip()):
                position = (i, j)
                if c == "+":
                    self._intersections.add(position)
                elif c == ">":
                    self._carts[position] = Directions.right, 0
                elif c == "<":
                    self._carts[position] = Directions.left, 0
                elif c == "^":
                    self._carts[position] = Directions.up, 0
                elif c == "v":
                    self._carts[position] = Directions.down, 0
                elif c == "\\":
                    self._bends[position] = {
                        Directions.left: Directions.up,
                        Directions.right: Directions.down,
                        Directions.up: Directions.left,
                        Directions.down: Directions.right,
                    }
                elif c == "/":
                    self._bends[position] = {
                        Directions.left: Directions.down,
                        Directions.right: Directions.up,
                        Directions.up: Directions.right,
                        Directions.down: Directions.left,
                    }
                elif c not in ["-", "|", " "]:
                    raise RuntimeError("Unknown track item", c)

    def first_collision(self):
        while len(self._collisions) == 0:
            self.tick()
        collision = self._collisions[0]
        return collision[1], collision[0]

    def last_standing(self):
        while len(self._carts) > 1:
            self.tick()
        last_standing = self._carts.keys()[0]
        return last_standing[1], last_standing[0]

    def tick(self):
        new_carts = {}
        carts = self._get_ordered_carts()
        while len(carts) > 0:
            coords = carts.pop(0)
            direction, turns = self._carts[coords]

            if direction == Directions.right:
                new_coords = coords[0], coords[1] + 1
            elif direction == Directions.left:
                new_coords = coords[0], coords[1] - 1
            elif direction == Directions.up:
                new_coords = coords[0] - 1, coords[1]
            elif direction == Directions.down:
                new_coords = coords[0] + 1, coords[1]
            else:
                raise RuntimeError()

            new_direction = direction
            new_turns = turns

            if new_coords in new_carts:
                self._collisions.append(new_coords)
                new_carts.pop(new_coords, None)
                continue
            elif new_coords in carts:
                self._collisions.append(new_coords)
                carts.remove(new_coords)
                continue
            elif new_coords in self._bends:
                new_direction = self._bends[new_coords][direction]
            elif new_coords in self._intersections:
                if turns % 3 == 0:
                    new_direction = {
                        Directions.left: Directions.down,
                        Directions.right: Directions.up,
                        Directions.up: Directions.left,
                        Directions.down: Directions.right,
                    }[direction]
                elif turns % 3 == 2:
                    new_direction = {
                        Directions.left: Directions.up,
                        Directions.right: Directions.down,
                        Directions.up: Directions.right,
                        Directions.down: Directions.left,
                    }[direction]
                new_turns += 1

            new_carts[new_coords] = new_direction, new_turns

        self._carts = new_carts

    def _get_ordered_carts(self):
        return sorted(self._carts)
