import unittest


class Labyrinth:
    def __init__(self, labyrinth_lines):
        self._lines = labyrinth_lines
        self._height = len(self._lines)
        self._width = max(map(len, self._lines))

    class OutOfBounds(Exception):
        pass

    def __getitem__(self, coordinates):
        x = coordinates[0]
        y = coordinates[1]
        if self._coordinates_in_bounds(x, y):
            raise Labyrinth.OutOfBounds
        line = self._lines[x]
        if y >= len(line):
            return " "
        return line[y]

    def find_entrance(self):
        for i, value in enumerate(self._lines[0]):
            if value == "|":
                return 0, i

    def _coordinates_in_bounds(self, x, y):
        return x < 0 or x >= self._height or y < 0 or y >= self._width


class TestLabyrinth(unittest.TestCase):
    def test_construct(self):
        labyrinth = Labyrinth([
            "|",
            "A",
        ])

        self.assertEqual(2, len(labyrinth._lines))

    def test_labyrinth_cannot_be_empty(self):
        self.assertRaises(ValueError, Labyrinth, [])

    def test_read_coordinates(self):
        labyrinth = Labyrinth([
            "| ",
            "A-",
        ])

        self.assertEqual("|", labyrinth[0, 0])
        self.assertEqual("A", labyrinth[1, 0])
        self.assertEqual("-", labyrinth[1, 1])
        self.assertEqual(" ", labyrinth[0, 1])

    def test_read_coordinates_excepts_out_of_bounds(self):
        labyrinth = Labyrinth([
            "|",
        ])

        def get_location(lab, x, y):
            return lab[x, y]

        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, 1, 0)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, 0, 1)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, -1, 0)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, 0, -1)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, -1, -1)

        labyrinth2 = Labyrinth([
            "|",
            "+"
        ])
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth2, 0, 1)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth2, 1, 1)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth2, 2, 0)

    def test_lines_with_different_lengths_are_filled(self):
        labyrinth = Labyrinth([
            "|",
            "A-",
        ])

        self.assertEqual(" ", labyrinth[0, 1])

    def test_find_entrance(self):
        labyrinth = Labyrinth([
            "| ",
            "A-",
        ])
        self.assertEqual((0, 0), labyrinth.find_entrance())

        labyrinth = Labyrinth([
            " |",
            "-A",
        ])
        self.assertEqual((0, 1), labyrinth.find_entrance())

        labyrinth = Labyrinth([
            "-A",
            " |"
        ])
        self.assertEqual(None, labyrinth.find_entrance())

