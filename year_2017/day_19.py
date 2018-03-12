import unittest
from aoc_utils import data_file


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
        self._found_letters = []
        self._steps = 1

    DOWN = 1, 0
    UP = -1, 0
    LEFT = 0, -1
    RIGHT = 0, 1

    def walk(self):
        try:
            while True:
                self.step()
        except Labyrinth.OutOfBounds:
            pass

    def step(self):
        self._step_towards(self._direction)

        current_value = self._labyrinth[self._position]
        if current_value == "+":
            self._choose_next_direction()
        elif 'A' <= current_value <= 'Z':
            self._found_letters.append(current_value)

        self._steps += 1

    def word(self):
        return "".join(self._found_letters)

    def steps(self):
        return self._steps

    def _step_towards(self, direction):
        self._position = self._position_towards(direction)

    def _position_towards(self, direction):
        return tuple(self._position[i] + direction[i] for i in (0, 1))

    def _choose_next_direction(self):
        def check_for_and_turn_to(for_directions, to_directions, next_marker):
            for for_direction in for_directions:
                if self._direction == for_direction:
                    for to_direction in to_directions:
                        to_value = self._value_towards(to_direction)
                        if to_value == next_marker or 'A' <= to_value <= 'Z':
                            self._direction = to_direction
                            return True
            return False

        up_down = (Walker.DOWN, Walker.UP)
        right_left = (Walker.LEFT, Walker.RIGHT)
        if (not check_for_and_turn_to(up_down, right_left, "-")
                and not check_for_and_turn_to(right_left, up_down, "|")):
            raise RuntimeError

    def _value_towards(self, direction):
        position = self._position_towards(direction)
        try:
            return self._labyrinth[position]
        except Labyrinth.OutOfBounds:
            return ' '


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

    def test_initialises_to_1_step(self):
        self.assertEqual(1, self.walker.steps())

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

    def test_step_registers_found_letters(self):
        def check_generated_word(expected_word, labyrinth_lines, num_steps):
            labyrinth = Labyrinth(labyrinth_lines)
            walker = Walker(labyrinth)
            for _ in range(num_steps):
                walker.step()

            self.assertEqual(expected_word, walker.word())

        check_generated_word("A", [
            " |",
            " A",
        ], num_steps=1)

        check_generated_word("AAA", [
            " |",
            " A",
            " A",
            " A",
        ], num_steps=3)

        check_generated_word("ABCD", [
            "|",
            "A",
            "+-B-+",
            "    |",
            "D-C-+",
        ], num_steps=12)

        check_generated_word("ABCDEFG", [
            "| G +-D+",
            "A | |  |",
            "| +F|E-+",
            "B   |",
            "+-C-+",
        ], num_steps=24)

    def test_example(self):
        with open(data_file(2017, "day_19_example.txt")) as f:
            labyrinth_lines = f.readlines()

        labyrinth = Labyrinth(labyrinth_lines)
        walker = Walker(labyrinth)
        walker.walk()

        self.assertEqual("ABCDEF", walker.word())
        self.assertEqual(38, walker.steps())

    def test_mine(self):
        with open(data_file(2017, "day_19_mine.txt")) as f:
            labyrinth_lines = f.readlines()

        labyrinth = Labyrinth(labyrinth_lines)
        walker = Walker(labyrinth)
        walker.walk()

        self.assertEqual("PBAZYFMHT", walker.word())
        self.assertEqual(16072, walker.steps())
