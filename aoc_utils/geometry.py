import itertools
import unittest


def add_coordinates(a, b):
    """Helper to add 2 coordinates vectors"""
    return tuple(u + v for u, v in zip(a, b))


def scale_coordinates(coordinates, factor):
    return tuple(u * factor for u in coordinates)


class TestMahattanDistance(unittest.TestCase):
    def test_one_dimension_distance_is_difference(self):
        self.assertEqual(0, manhattan_distance((42,), (42,)))
        self.assertEqual(27, manhattan_distance((69,), (42,)))
        self.assertEqual(27, manhattan_distance((42,), (69,)))
        self.assertEqual(27, manhattan_distance((-69,), (-42,)))
        self.assertEqual(27, manhattan_distance((-42,), (-69,)))
        self.assertEqual(111, manhattan_distance((-42,), (69,)))
        self.assertEqual(111, manhattan_distance((42,), (-69,)))


def manhattan_distance(u, v):
    assert len(u) == len(v)
    return sum(abs(a-b) for a, b in zip(u, v))


class TestHyperCube(unittest.TestCase):
    def test_overlaps(self):
        segment_0_2 = HyperCube((0,), (2,))
        segment_1_3 = HyperCube((1,), (3,))
        segment_2_4 = HyperCube((2,), (4,))
        segment_m1_1 = HyperCube((-1,), (1,))
        self.assertTrue(segment_0_2.overlaps(segment_1_3))
        self.assertFalse(segment_0_2.overlaps(segment_2_4))
        self.assertTrue(segment_0_2.overlaps(segment_m1_1))
        self.assertTrue(segment_1_3.overlaps(segment_2_4))
        self.assertFalse(segment_1_3.overlaps(segment_m1_1))
        self.assertFalse(segment_2_4.overlaps(segment_m1_1))

        plane_0_2 = HyperCube((0, 0), (2, 2))
        plane_1_3 = HyperCube((1, 1), (3, 3))
        plane_2_4 = HyperCube((2, 2), (4, 4))
        plane_m1_1 = HyperCube((-1, -1), (1, 1))
        self.assertTrue(plane_0_2.overlaps(plane_1_3))
        self.assertFalse(plane_0_2.overlaps(plane_2_4))
        self.assertTrue(plane_0_2.overlaps(plane_m1_1))
        self.assertTrue(plane_1_3.overlaps(plane_2_4))
        self.assertFalse(plane_1_3.overlaps(plane_m1_1))
        self.assertFalse(plane_2_4.overlaps(plane_m1_1))

    def test_includes(self):
        segment_0_2 = HyperCube((0,), (2,))
        segment_1_2 = HyperCube((1,), (2,))
        segment_1_3 = HyperCube((1,), (3,))
        self.assertTrue(segment_0_2.includes(segment_0_2))
        self.assertTrue(segment_0_2.includes(segment_1_2))
        self.assertFalse(segment_0_2.includes(segment_1_3))
        self.assertFalse(segment_1_2.includes(segment_1_3))
        self.assertTrue(segment_1_3.includes(segment_1_2))
        self.assertFalse(segment_1_3.includes(segment_0_2))

        plane_0_2 = HyperCube((0, 0), (2, 2))
        plane_1_2 = HyperCube((1, 1), (2, 2))
        plane_1_3 = HyperCube((1, 1), (3, 3))
        self.assertTrue(plane_0_2.includes(plane_0_2))
        self.assertTrue(plane_0_2.includes(plane_1_2))
        self.assertFalse(plane_0_2.includes(plane_1_3))
        self.assertFalse(plane_1_2.includes(plane_0_2))
        self.assertFalse(plane_1_2.includes(plane_1_3))
        self.assertFalse(plane_1_3.includes(plane_0_2))
        self.assertTrue(plane_1_3.includes(plane_1_2))

    def test_eq(self):
        self.assertEqual(HyperCube((1, 2, 3), (4, 5, 6)), HyperCube((1, 2, 3), (4, 5, 6)))

    def test_contains(self):
        segment_1_2 = HyperCube((1,), (2,))

        self.assertFalse(segment_1_2.contains((-1,)))
        self.assertFalse(segment_1_2.contains((0,)))
        self.assertTrue(segment_1_2.contains((1,)))
        self.assertTrue(segment_1_2.contains((2,)))
        self.assertFalse(segment_1_2.contains((3,)))

        plane_1_2 = HyperCube((1, 1), (2, 2))

        self.assertFalse(plane_1_2.contains((-1, 0)))
        self.assertFalse(plane_1_2.contains((0, -1)))
        self.assertFalse(plane_1_2.contains((0, 0)))
        self.assertFalse(plane_1_2.contains((0, 1)))
        self.assertFalse(plane_1_2.contains((1, 0)))
        self.assertTrue(plane_1_2.contains((1, 1)))
        self.assertTrue(plane_1_2.contains((1, 2)))
        self.assertTrue(plane_1_2.contains((2, 1)))
        self.assertTrue(plane_1_2.contains((2, 2)))
        self.assertFalse(plane_1_2.contains((3, 2)))
        self.assertFalse(plane_1_2.contains((2, 3)))
        self.assertFalse(plane_1_2.contains((3, 3)))

    def test_collide(self):
        segment_0_2 = HyperCube((0,), (2,))
        segment_0_1 = HyperCube((0,), (1,))
        segment_1_2 = HyperCube((1,), (2,))
        segment_1_3 = HyperCube((1,), (3,))
        segment_2_3 = HyperCube((2,), (3,))

        self.assertDictEqual({segment_0_2: 2}, collide(segment_0_2, 1, segment_0_2, 1))
        segment_0_2_again = HyperCube((0,), (2,))
        self.assertDictEqual({segment_0_2: 2}, collide(segment_0_2, 1, segment_0_2_again, 1))

        self.assertDictEqual(
            {segment_0_2: 1, segment_2_3: 1},
            collide(segment_0_2, 1, segment_2_3, 1),
        )

        self.assertEqual(
            {segment_0_1: 1, segment_1_2: 2},
            collide(segment_0_2, 1, segment_1_2, 1),
        )

        self.assertEqual(
            {segment_0_1: 1, segment_1_2: 2, segment_2_3: 1},
            collide(segment_0_2, 1, segment_1_3, 1),
        )

        self.assertEqual(
            {segment_0_1: 3, segment_1_2: 8, segment_2_3: 5},
            collide(segment_0_2, 3, segment_1_3, 5),
        )

        plane_0_2 = HyperCube((0, 0), (2, 2))
        plane_1_2 = HyperCube((1, 1), (2, 2))
        plane_1_3 = HyperCube((1, 1), (3, 3))

        self.assertEqual(
            {
                HyperCube((0, 0), (1, 1)): 1,
                HyperCube((1, 0), (2, 1)): 1,
                HyperCube((0, 1), (1, 2)): 1,
                HyperCube((1, 1), (2, 2)): 2,
            },
            collide(plane_0_2, 1, plane_1_2, 1),
        )

        self.assertEqual(
            {
                HyperCube((0, 0), (1, 1)): 1,
                HyperCube((1, 0), (2, 1)): 1,
                HyperCube((2, 0), (3, 1)): 0,
                HyperCube((0, 1), (1, 2)): 1,
                HyperCube((1, 1), (2, 2)): 2,
                HyperCube((2, 1), (3, 2)): 1,
                HyperCube((0, 2), (1, 3)): 0,
                HyperCube((1, 2), (2, 3)): 1,
                HyperCube((2, 2), (3, 3)): 1,
            },
            collide(plane_0_2, 1, plane_1_3, 1),
        )


class HyperCube:
    def __init__(self, lower, upper):
        self.dimension = len(lower)
        self.lower = lower
        self.upper = upper

    def overlaps(self, other):
        return all(
            self.lower[i] < other.upper[i] and self.upper[i] > other.lower[i]
            for i in range(self.dimension)
        )

    def includes(self, other):
        return all(
            self.lower[i] <= other.lower[i] and self.upper[i] >= other.upper[i]
            for i in range(self.dimension)
        )

    def contains(self, coordinates):
        return all(self.lower[i] <= coordinates[i] <= self.upper[i] for i in range(self.dimension))

    def __eq__(self, other):
        return self.lower == other.lower and self.upper == other.upper

    def __hash__(self):
        return hash((self.dimension, self.lower, self.upper))

    def __repr__(self):
        return f'Cube({self.lower}, {self.upper})'


def collide(cube_a, weight_a, cube_b, weight_b):
    if cube_a == cube_b:
        return {cube_a: weight_a + weight_b}
    elif not cube_a.overlaps(cube_b):
        return {cube_a: weight_a, cube_b: weight_b}
    else:
        coordinate_options = [
            list(sorted({cube_a.lower[i], cube_a.upper[i], cube_b.lower[i], cube_b.upper[i]}))
            for i in range(cube_a.dimension)
        ]
        pairs = make_pairs(coordinate_options)

        result = {}
        for option in iterate_pairs(pairs):
            lower, upper = pairs_to_bounds(option)
            cube = HyperCube(lower, upper)
            weight = 0
            if cube_a.includes(cube):
                weight += weight_a
            if cube_b.includes(cube):
                weight += weight_b
            result[cube] = weight
        return result


class TestPairs(unittest.TestCase):
    def test_make_pairs(self):
        self.assertEqual(
            [[(0, 1), (1, 2)]],
            make_pairs([[0, 1, 2]])
        )
        self.assertEqual(
            [
                [(0, 1), (1, 2)],
                [(3, 4), (4, 5)],
            ],
            make_pairs([[0, 1, 2], [3, 4, 5]])
        )

    def test_iterate_pairs(self):
        self.assertSetEqual(
            {
                ((0, 1),),
                ((1, 2),),
                ((2, 3),),
            },
            frozenset(iterate_pairs([
                [(0, 1), (1, 2), (2, 3)],
            ])),
        )
        self.assertSetEqual(
            {
                ((0, 1), (3, 4)),
                ((0, 1), (4, 5)),
                ((1, 2), (3, 4)),
                ((1, 2), (4, 5)),
            },
            frozenset(iterate_pairs([
                [(0, 1), (1, 2)],
                [(3, 4), (4, 5)],
            ])),
        )

    def test_pairs_to_bounds(self):
        self.assertEqual(
            ((0, ), (1, )),
            pairs_to_bounds(((0, 1),))
        )
        self.assertEqual(
            ((0, 3), (1, 4)),
            pairs_to_bounds(((0, 1), (3, 4)))
        )
        self.assertEqual(
            ((0, 1, 2), (3, 4, 5)),
            pairs_to_bounds(((0, 3), (1, 4), (2, 5)))
        )


def make_pairs(options):
    result = []
    for dimension_options in options:
        dimension_pairs = [
            (dimension_options[i], dimension_options[i+1])
            for i in range(len(dimension_options) - 1)
        ]
        result.append(dimension_pairs)
    return result


def iterate_pairs(pairs):
    return itertools.product(*pairs)


def pairs_to_bounds(pair):
    lower = tuple(dimension_bounds[0] for dimension_bounds in pair)
    upper = tuple(dimension_bounds[1] for dimension_bounds in pair)
    return lower, upper
