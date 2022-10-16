import copy
import itertools
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

    @unittest.skip("Still a little too slow")
    def test_cabinet_bruteforce(self):
        program = list(map(int, data_text(2019, "day_13_mine.txt").split(",")))
        program[0] = 2

        inputs = []
        # inputs = [(0, 3), (-1, 4), (0, 30), (-1, 8), (0, 2), (-1, 6), (0, 8), (1, 8), (0, 2), (1, 8), (0, 40), (-1, 10), (0, 6), (-1, 4), (0, 6), (1, 8), (0, 8), (-1, 10), (0, 42), (1, 16), (0, 114), (1, 14), (0, 166), (-1, 16), (0, 16), (1, 2), (0, 58), (-1, 2), (0, 30), (1, 22), (0, 46), (-1, 12), (0, 158), (-1, 16), (0, 28), (-1, 4), (0, 32), (1, 28), (0, 36), (-1, 32), (0, 18), (1, 32), (0, 28), (-1, 32), (1, 18), (0, 2), (1, 4), (0, 50), (-1, 4), (0, 50), (1, 4), (0, 28), (1, 6), (0, 26), (-1, 16), (0, 30), (1, 16), (0, 16), (-1, 26), (0, 6), (1, 32), (-1, 24), (0, 20), (1, 20), (0, 52), (-1, 20), (0, 30), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 26), (1, 24), (0, 8), (-1, 32), (1, 30), (0, 2), (-1, 20), (0, 44), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 8), (1, 2), (0, 30), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 42), (1, 24), (0, 8), (-1, 32), (1, 30), (0, 2), (-1, 20), (0, 46), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 8), (1, 14), (0, 18), (-1, 4), (0, 28), (-1, 6), (0, 26), (1, 16), (0, 16), (-1, 26), (0, 6), (1, 32), (0, 42), (-1, 32), (1, 26), (0, 6), (-1, 16), (0, 16), (1, 6), (0, 26), (1, 4), (0, 28), (-1, 14), (0, 18), (1, 24), (0, 8), (-1, 32), (1, 30), (0, 2), (-1, 20), (0, 48), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 8), (1, 14), (0, 18), (-1, 4), (0, 28), (-1, 6), (0, 26), (1, 16), (0, 16), (-1, 26), (0, 6), (1, 32), (-1, 28), (0, 36), (1, 28), (0, 4), (-1, 32), (1, 26), (0, 6), (-1, 16), (0, 16), (1, 6), (0, 26), (1, 4), (0, 28), (-1, 14), (0, 18), (1, 24), (0, 8), (-1, 32), (1, 30), (0, 2), (-1, 20), (0, 12), (1, 10), (0, 54), (-1, 10), (0, 22), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 8), (1, 14), (0, 18), (-1, 4), (0, 28), (-1, 6), (0, 26), (1, 16), (0, 16), (-1, 26), (0, 6), (1, 32), (-1, 28), (0, 46), (1, 28), (0, 4), (-1, 32), (1, 26), (0, 6), (-1, 16), (0, 16), (1, 6), (0, 26), (1, 4), (0, 28), (-1, 14), (0, 18), (1, 24), (0, 8), (-1, 32), (1, 30), (0, 2), (-1, 20), (0, 12), (1, 10), (0, 54), (-1, 10), (0, 22), (1, 20), (0, 12), (-1, 30), (0, 2), (1, 32), (-1, 24), (0, 8), (1, 14), (0, 18), (-1, 4), (0, 28), (-1, 6), (0, 26), (1, 16), (0, 16), (-1, 26), (0, 6), (1, 32), (-1, 28), (0, 4), (1, 18), (0, 14), (-1, 8), (0, 24), (-1, 2), (0, 30), (1, 12), (0, 20), (-1, 10), (0, 1)]

        bruteforcer = ArcadeBruteForcer(program, inputs)
        arcade = bruteforcer.bruteforce(verbose=False)

        self.assertEqual(0, arcade.display.count_of(BLOCK))
        self.assertEqual(8942, arcade.display.score)


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
        return [
            coordinates
            for coordinates, value in self._tiles.items()
            if value == tile_id
        ]

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


class ArcadeBruteForcer:
    def __init__(self, program, initial_inputs=None):
        self.program = program
        self.committed_inputs = initial_inputs or []

    def bruteforce(self, verbose=False):
        arcade = ArcadeCabinet(
            program=self.program,
            inputs=list(self.decompressed_inputs()),
        )
        next_inputs = []
        while True:
            arcade.processor.input_values.extend(self.decompress(next_inputs))
            try:
                arcade.run_game(echo=False)
            except GameOver:
                break
            except InputNeeded:
                pass
            else:
                break
            finally:
                if verbose:
                    print(arcade.display.score, self.committed_inputs)

            delta_x, experiment_time = self.run_experiment(arcade)
            next_inputs = self.calculate_next_inputs(delta_x, experiment_time)
            self.committed_inputs.extend(next_inputs)

        self.recompress_inputs()
        return arcade

    def run_experiment(self, arcade):
        experiment_time = 0

        test_arcade = copy.deepcopy(arcade)
        while True:
            test_arcade.processor.input_values.append(0)
            try:
                test_arcade.run_game(echo=False)
            except GameOver:
                break
            except InputNeeded:
                pass
            finally:
                experiment_time += 1
                ball = test_arcade.display.search(BALL)[0]
                paddle = test_arcade.display.search(PADDLE)[0]

            if ball[1] == paddle[1] - 1:
                break

        delta_x = ball[0] - paddle[0]
        return delta_x, experiment_time

    @staticmethod
    def calculate_next_inputs(delta_x, experiment_time):
        time_to_bounce = experiment_time + 1

        if time_to_bounce < abs(delta_x):
            raise RuntimeError("impossible")

        if delta_x == 0:
            next_inputs = [(0, time_to_bounce)]
        else:
            direction = delta_x // abs(delta_x)
            moves = abs(delta_x)
            next_inputs = [
                (direction, moves),
                (0, time_to_bounce - moves),
            ]
        return next_inputs

    def decompressed_inputs(self):
        return self.decompress(self.committed_inputs)

    @staticmethod
    def decompress(compressed_inputs):
        return itertools.chain.from_iterable(
            itertools.repeat(value, times)
            for value, times in compressed_inputs
        )

    def recompress_inputs(self):
        self.committed_inputs = [
            (group[0], sum(1 for _ in (group[1])))
            for group in itertools.groupby(self.decompressed_inputs())
        ]
