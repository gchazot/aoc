import unittest
from aoc_utils.data import data_lines

CENTER_OF_MASS = "COM"
EXAMPLE_LINES_1 = [
    "COM)B",
    "B)C",
    "C)D",
    "D)E",
    "E)F",
    "B)G",
    "G)H",
    "D)I",
    "E)J",
    "J)K",
    "K)L",
]
EXAMPLE_LINES_2 = [
    "COM)B",
    "B)C",
    "C)D",
    "D)E",
    "E)F",
    "B)G",
    "G)H",
    "D)I",
    "E)J",
    "J)K",
    "K)L",
    "K)YOU",
    "I)SAN",
]


class TestParseOrbitMap(unittest.TestCase):
    def test_parse_empty(self):
        map_lines = []
        direct_orbits = parse_orbit_map(map_lines)
        self.assertEqual(0, len(direct_orbits))

    def test_parse_example(self):
        map_lines = EXAMPLE_LINES_1
        direct_orbits = parse_orbit_map(map_lines)
        self.assertEqual(11, len(direct_orbits))
        for orbit in direct_orbits:
            self.assertIsInstance(orbit, list)
            self.assertEqual(2, len(orbit))

    def test_parse_mine(self):
        map_lines = data_lines(2019, 'day_06_mine.txt')
        direct_orbits = parse_orbit_map(map_lines)
        self.assertEqual(1376, len(direct_orbits))


def parse_orbit_map(map_lines):
    orbits = []
    for line in map_lines:
        orbits.append(line.split(")"))
    return orbits


class TestOrbitMap(unittest.TestCase):
    def test_network(self):
        direct_orbits = parse_orbit_map(EXAMPLE_LINES_1)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(12, len(orbits.centers))

    def test_checksum_example(self):
        direct_orbits = parse_orbit_map(EXAMPLE_LINES_1)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(42, orbits.checksum)

    def test_checksum_mine(self):
        map_lines = data_lines(2019, 'day_06_mine.txt')
        direct_orbits = parse_orbit_map(map_lines)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(249308, orbits.checksum)

    def test_chain_example(self):
        direct_orbits = parse_orbit_map(EXAMPLE_LINES_1)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(
            [CENTER_OF_MASS, 'B'],
            orbits.com_path('G'),
        )
        self.assertEqual(
            [CENTER_OF_MASS, 'B', 'G'],
            orbits.com_path('H'),
        )
        self.assertEqual(
            [CENTER_OF_MASS, 'B', 'C', 'D', 'E', 'J'],
            orbits.com_path('K'),
        )

    def test_distance_example_1(self):
        direct_orbits = parse_orbit_map(EXAMPLE_LINES_1)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(3, orbits.distance('L', 'I'))
        self.assertEqual(3, orbits.distance('I', 'L'))
        self.assertEqual(6, orbits.distance('L', 'H'))
        self.assertEqual(6, orbits.distance('H', 'L'))

    def test_distance_example_2(self):
        direct_orbits = parse_orbit_map(EXAMPLE_LINES_2)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(4, orbits.distance('YOU', 'SAN'))
        self.assertEqual(4, orbits.distance('SAN', 'YOU'))

    def test_distance_mine(self):
        map_lines = data_lines(2019, 'day_06_mine.txt')
        direct_orbits = parse_orbit_map(map_lines)
        orbits = OrbitMap(direct_orbits)
        self.assertEqual(349, orbits.distance('YOU', 'SAN'))
        self.assertEqual(349, orbits.distance('SAN', 'YOU'))


class OrbitMap:
    def __init__(self, direct_orbits):
        self.centers = {}
        self.orbiting = {}
        for center, object in direct_orbits:
            self.orbiting[object] = center
            self.centers.setdefault(center, set())
            self.centers.setdefault(object, set())
            self.centers[center].add(object)

    @property
    def checksum(self):
        distances = {'COM': 0}
        centers = ['COM']
        while centers:
            next_objects = []
            for center in centers:
                for orbiting in self.centers[center]:
                    distances[orbiting] = 1 + distances[center]
                    next_objects.append(orbiting)
            centers = next_objects

        return sum(distances.values())

    def distance(self, a, b):
        chain_a = self.com_path(a)
        chain_b = self.com_path(b)

        while chain_a and chain_b and chain_a[0] == chain_b[0]:
            chain_a.pop(0)
            chain_b.pop(0)

        return len(chain_a) + len(chain_b)

    def com_path(self, destination):
        steps = []

        next_object = destination
        while next_object != CENTER_OF_MASS:
            next_object = self.orbiting[next_object]
            steps.insert(0, next_object)

        return steps

