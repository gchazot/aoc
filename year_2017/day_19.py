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
            raise Labyrinth.OutOfBounds
        if line[y] == " ":
            raise Labyrinth.OutOfBounds
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

    def test_read_empty_position_raises_exception(self):
        labyrinth = Labyrinth([
            " |",
            "++",
            "+",
        ])

        def get_location(lab, x, y):
            return lab[x, y]

        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, 0, 0)
        self.assertRaises(Labyrinth.OutOfBounds, get_location, labyrinth, 2, 1)

    def test_read_coordinates_out_of_bounds_raises_exception(self):
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


class Walker:
    def __init__(self, labyrinth):
        self._labyrinth = labyrinth
        self._position = self._labyrinth.find_entrance()
        self._direction = Walker.DOWN

    DOWN = 1, 0
    UP = -1, 0
    LEFT = 0, -1
    RIGHT = 0, 1

    def step(self):
        self._step_towards(self._direction)
        if self._labyrinth[self._position] == "+":
            if self._direction == Walker.DOWN or self._direction == Walker.UP:
                if self._value_towards(Walker.LEFT) == "-":
                    self._direction = Walker.LEFT
                elif self._value_towards(Walker.RIGHT) == "-":
                    self._direction = Walker.RIGHT
                else:
                    raise RuntimeError
            elif self._direction == Walker.LEFT or self._direction == Walker.RIGHT:
                if self._value_towards(Walker.UP) == "|":
                    self._direction = Walker.UP
                elif self._value_towards(Walker.DOWN) == "|":
                    self._direction = Walker.DOWN
                else:
                    raise RuntimeError
            else:
                raise RuntimeError

    def _step_towards(self, direction):
        self._position = self._position_towards(direction)

    def _value_towards(self, direction):
        position = self._position_towards(direction)
        try:
            return self._labyrinth[position]
        except Labyrinth.OutOfBounds:
            return None

    def _position_towards(self, direction):
        return tuple(self._position[i] + direction[i] for i in (0, 1))


class TestWalker(unittest.TestCase):
    def setUp(self):
        self.labyrinth = Labyrinth([
            " |",
            "-+"
        ])
        self.walker = Walker(self.labyrinth)

    def test_initialises_to_entrance(self):
        self.assertEqual((0, 1), self.walker._position)

        other_labyrinth = Labyrinth([
            "|",
            "+-"
        ])
        other_walker = Walker(other_labyrinth)
        self.assertEqual((0, 0), other_walker._position)

    def test_initialises_to_going_down(self):
        self.assertEqual(Walker.DOWN, self.walker._direction)

    def test_step_towards_updates_position(self):
        self.assertEqual(self.walker._position, (0, 1))

        self.walker._step_towards(Walker.LEFT)
        self.assertEqual((0, 0), self.walker._position)

        self.walker._step_towards(Walker.DOWN)
        self.assertEqual((1, 0), self.walker._position)

        self.walker._step_towards(Walker.RIGHT)
        self.assertEqual((1, 1), self.walker._position)

        self.walker._step_towards(Walker.UP)
        self.assertEqual((0, 1), self.walker._position)

    def test_step_progresses_in_current_direction(self):
        self.walker.step()
        self.assertEqual(self.walker._position, (1, 1))

    def test_step_updates_direction_at_corner(self):
        def check_direction_sequence(labyrinth, directions):
            walker = Walker(labyrinth)
            for expected_direction in directions:
                self.assertEqual(expected_direction, walker._direction)
                walker.step()

        check_direction_sequence(Labyrinth([
            "|",
            "+-+",
            "  |",
        ]), [Walker.DOWN, Walker.RIGHT, Walker.RIGHT, Walker.DOWN])

        check_direction_sequence(Labyrinth([
            "  |",
            "+-+",
            "|",
        ]), [Walker.DOWN, Walker.LEFT, Walker.LEFT, Walker.DOWN])

        check_direction_sequence(Labyrinth([
            "| +-",
            "| |",
            "+-+",
        ]), [Walker.DOWN, Walker.DOWN, Walker.RIGHT, Walker.RIGHT, Walker.UP, Walker.UP, Walker.RIGHT])

        check_direction_sequence(Labyrinth([
            "-+ |",
            " | |",
            " +-+",
        ]), [Walker.DOWN, Walker.DOWN, Walker.LEFT, Walker.LEFT, Walker.UP, Walker.UP, Walker.LEFT])
