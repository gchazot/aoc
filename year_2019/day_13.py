import unittest
try:
    from mock import MagicMock
except ImportError:
    from unittest.mock import MagicMock

from aoc_utils.char_map import CharMap
from aoc_utils.data import data_text
from aoc_utils.geometry import add_coordinates
from year_2019.int_code_processor import IntCodeProcessor, instructions_day_09, InputNeeded


EMPTY = 0
WALL = 1
BLOCK = 2
PADDLE = 3
BALL = 4

SPRITES = [".", "#", "-", "=", "o"]


class TestArcade(unittest.TestCase):
    def test_display(self):
        display = ArcadeDisplay()
        self.assertEqual(0, display.count)
        self.assertEqual(0, display.frames)
        self.assertIsInstance(display.input_values, DisplayConnector)

        display.receive((33, 42), WALL)
        self.assertEqual(1, display.count)
        self.assertEqual(1, display.frames)
        self.assertEqual(WALL, display.get((33, 42)))

        display.receive((33, 42), BLOCK)
        self.assertEqual(1, display.count)
        self.assertEqual(2, display.frames)
        self.assertEqual(BLOCK, display.get((33, 42)))

        display.receive((33, 42), WALL)
        self.assertEqual(1, display.count)
        self.assertEqual(3, display.frames)
        self.assertEqual(WALL, display.get((33, 42)))

        display.receive((33, 42), EMPTY)
        self.assertEqual(0, display.count)
        self.assertEqual(4, display.frames)
        self.assertEqual(EMPTY, display.get((33, 42)))

        display.receive((-1, 0), 65789)
        self.assertEqual(65789, display.score)
        self.assertEqual(4, display.frames)

    def test_display_search(self):
        display = ArcadeDisplay()

        for i in range(5):
            display.receive((i, 0), WALL)
            display.receive((0, i+1), WALL)
            display.receive((4, i+1), WALL)

        for i in range(1, 4):
            for y in range(1, 3):
                if (i+y) % 2 == 0:
                    display.receive((i, y), BLOCK)

        display.receive((3, 3), BALL)
        display.receive((2, 5), PADDLE)
        self.assertEqual([(3, 3)], display.search(BALL))
        self.assertEqual([(2, 5)], display.search(PADDLE))

    def test_connector(self):
        fake_display = MagicMock()
        connector = DisplayConnector(display=fake_display)

        connector.append(1)
        fake_display.receive.assert_not_called()
        connector.append(2)
        fake_display.receive.assert_not_called()
        connector.append(PADDLE)
        fake_display.receive.assert_called_once_with((1, 2), PADDLE)
        fake_display.reset_mock()

        connector.append(6)
        fake_display.receive.assert_not_called()
        connector.append(5)
        fake_display.receive.assert_not_called()
        connector.append(BALL)
        fake_display.receive.assert_called_once_with((6, 5), BALL)

    def test_cabinet(self):
        program = list(map(int, data_text(2019, "day_13_mine.txt").split(",")))

        arcade = ArcadeCabinet(program=program)

        with self.assertRaises(GameOver):
            arcade.run_game()

        self.assertEqual(0, len(arcade.processor.output_values))

        self.assertEqual(253, arcade.display.count)
        self.assertEqual(173, arcade.display.count_of(2))
        self.assertEqual(0, arcade.display.score)


class ArcadeDisplay:
    def __init__(self):
        self.input_values = DisplayConnector(self)
        self._tiles = {}
        self.score = 0
        self.frames = 0
        self._render_cache_key = None
        self._render_cache = None

    def receive(self, coordinates, value):
        if coordinates == (-1, 0):
            self.score = value
        else:
            self.frames += 1
            if value == 0:
                self._tiles.pop(coordinates, None)
            else:
                self._tiles[coordinates] = value

    @property
    def count(self):
        return len(self._tiles)

    def count_of(self, tile_id):
        return sum(1 for tile in self._tiles.values() if tile == tile_id)

    def get(self, coordinates):
        return self._tiles.get(coordinates, 0)

    def echo(self):
        rendered = self._render()
        rendered.echo()
        print("Score:{:>5d}  Count:{:>4d}  Frame:{:>5d}".format(
            self.score,
            rendered.count_values([SPRITES[BLOCK]]),
            self.frames,
        ))

    def search(self, tile_id):
        rendered = self._render()
        return list(rendered.search(SPRITES[tile_id]))

    def _render(self):
        if self._render_cache is not None or self._tiles != self._render_cache_key:
            min_x = min(x for x, y in self._tiles.keys())
            min_y = min(y for x, y in self._tiles.keys())
            offset = (-min_x, -min_y)
            width = max(x for x, y in self._tiles.keys()) - min_x + 1
            height = max(y for x, y in self._tiles.keys()) - min_y + 1
            renderer = CharMap(width_height=(width, height))
            for coordinates, tile_id in self._tiles.items():
                position = add_coordinates(coordinates, offset)
                renderer[position] = SPRITES[tile_id]

            self._render_cache = renderer
            self._render_cache_key = self._tiles.copy()

        return self._render_cache


class DisplayConnector(list):
    def __init__(self, display):
        self._display = display
        self._buffer = []
        super(DisplayConnector, self).__init__()

    def append(self, obj):
        self._buffer.append(obj)
        if len(self._buffer) == 3:
            coordinates = self._buffer[0], self._buffer[1]
            value = self._buffer[2]
            self._display.receive(coordinates, value)
            self._buffer = []


class ArcadeCabinet:
    def __init__(self, program, inputs=None):
        self.display = ArcadeDisplay()
        self.processor = IntCodeProcessor(
            program, instructions_day_09,
            input_values=inputs,
            output_values=self.display.input_values,
        )

    def run_game(self, echo=False):
        try:
            self.processor.execute()
        except InputNeeded:
            raise
        else:
            raise GameOver
        finally:
            if self.display.count_of(BLOCK) == 0:
                raise GameOver
            if echo:
                self.display.echo()


class GameOver(Exception):
    pass
