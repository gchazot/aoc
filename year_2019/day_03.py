import unittest
from aoc_utils.data import data_lines
from aoc_utils.geometry import add_coordinates, manhattan_distance


class TestTwistedWires(unittest.TestCase):
    def test_parse_wire_1(self):
        locations = list(parse_wire('R8,U5,L5,D3'))

        self.assertEqual(21, len(locations))
        self.assertIn((3, 2), locations)

        self.assertIn((3, 3), locations)
        self.assertIn((6, 5), locations)

    def test_parse_wire_2(self):
        locations = list(parse_wire('U7,R6,D4,L4'))

        self.assertEqual(21, len(locations))
        self.assertIn((2, 3), locations)

        self.assertIn((3, 3), locations)
        self.assertIn((6, 5), locations)

    def test_parse_wires(self):
        wires_gen = parse_wires(['R8,U5,L5,D3', 'U7,R6,D4,L4'])

        wires = [list(locations) for locations in wires_gen]

        self.assertEqual(2, len(wires))
        self.assertEqual(21, len(wires[0]))
        self.assertEqual(21, len(wires[1]))

    def test_find_closest_manhattan_cross_examples(self):
        example_1 = Wires(['R8,U5,L5,D3', 'U7,R6,D4,L4'])
        self.assertEqual(((3, 3), 6), example_1.closest_manhattan_cross())

        example_2 = Wires(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83'])
        self.assertEqual(159, example_2.closest_manhattan_cross()[1])

        example_3 = Wires([
            'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
            'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7',
        ])
        self.assertEqual(135, example_3.closest_manhattan_cross()[1])

    def test_find_closest_steps_cross(self):
        example_1 = Wires(['R8,U5,L5,D3', 'U7,R6,D4,L4'])
        self.assertEqual(((6, 5), 30), example_1.closest_steps_cross())

        example_2 = Wires(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83'])
        self.assertEqual(610, example_2.closest_steps_cross()[1])

        example_3 = Wires([
            'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
            'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7',
        ])
        self.assertEqual(410, example_3.closest_steps_cross()[1])

    def test_find_closest_manhattan_cross_mine(self):
        my_wires = Wires(data_lines(2019, 'day_03_mine.txt'))
        self.assertEqual(((4240, -1117), 5357), my_wires.closest_manhattan_cross())

    def test_find_closest_manhattan_cross_mine(self):
        my_wires = Wires(data_lines(2019, 'day_03_mine.txt'))
        self.assertEqual(((4240, -1272), 101956), my_wires.closest_steps_cross())


class Wires:
    def __init__(self, lines):
        wires_gen = parse_wires(lines)
        self.wire_lists = [list(locations) for locations in wires_gen]

    def closest_manhattan_cross(self):
        wires = [frozenset(wire) for wire in self.wire_lists]

        intersections = wires[0].intersection(wires[1])
        distances = {
            intersection: manhattan_distance((0, 0), intersection)
            for intersection in intersections
        }

        return min(distances.items(), key=lambda k_v: k_v[1])

    def closest_steps_cross(self):
        location_distances = []
        for wire in self.wire_lists:
            location_distances.append({})
            for distance, location in enumerate(wire):
                location_distances[-1].setdefault(location, distance + 1)

        wires = [frozenset(wire) for wire in self.wire_lists]
        intersections = wires[0].intersection(wires[1])

        distances = {
            intersection: location_distances[0][intersection] + location_distances[1][intersection]
            for intersection in intersections
        }

        return min(distances.items(), key=lambda k_v: k_v[1])

steps = {
    'R': (1, 0),
    'U': (0, 1),
    'L': (-1, 0),
    'D': (0, -1),
}


def parse_wires(lines):
    for line in lines:
        yield parse_wire(line)


def parse_wire(wire, start=(0, 0)):
    segments = wire.split(',')
    position = list(start)
    for segment in segments:
        direction = segment[0]
        step = steps[direction]
        num_steps = int(segment[1:])

        for i in range(num_steps):
            position = add_coordinates(position, step)
            yield position
