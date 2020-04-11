from __future__ import print_function

from unittest import TestCase
import itertools
try:
    from mock import MagicMock
except ImportError:
    from unittest.mock import MagicMock

from aoc_utils.data import data_lines


class Screen:
    def __init__(self, height, width):
        self.height = height
        self.width = width
        self.pixels = [[False for _ in range(width)] for _ in range(height)]

    def __iter__(self):
        return itertools.chain(*self.pixels)

    def __getitem__(self, coordinates):
        x, y = coordinates
        return self.pixels[y][x]

    def rect(self, width, height):
        for i in range(height):
            for j in range(width):
                self.pixels[i][j] = True

    def rotate_row(self, row, steps):
        for step in range(steps):
            first = self.pixels[row][-1]
            for i in range(self.width - 1):
                self.pixels[row][-1 - i] = self.pixels[row][-2 - i]
            self.pixels[row][0] = first

    def rotate_column(self, column, steps):
        for step in range(steps):
            first = self.pixels[-1][column]
            for j in range(self.height - 1):
                self.pixels[-1 - j][column] = self.pixels[-2 - j][column]
            self.pixels[0][column] = first

    def display(self):
        print("\n".join(self.print()))

    def print(self):
        def show_pixel(on_off):
            return [" ", "#"][on_off]
        for row in self.pixels:
            yield "".join(map(show_pixel, row))


class TestScreen(TestCase):
    def setUp(self):
        self.num_rows, self.num_cols = 3, 7
        self.screen = Screen(self.num_rows, self.num_cols)

    def check_only_rectangle_is_on(self, width, height):
        for n, pixel in enumerate(self.screen):
            x, y = n % self.num_cols, int(n / self.num_cols)
            if x < width and y < height:
                self.assertTrue(pixel, msg="{2}[{0},{1}] should be True".format(x, y, n))
            else:
                self.assertFalse(pixel, msg="{2}[{0},{1}] should be False".format(x, y, n))

    def checkPixel(self, expected, x, y):
        self.assertEqual(expected, self.screen[x, y],
                         msg="[{0},{1}] should be {2}".format(x, y, expected))

    def test_screen_initially_blank(self):
        for pixel in self.screen:
            self.assertFalse(pixel)

    def test_can_create_rectangle_size_1_1(self):
        self.screen.rect(1, 1)
        self.check_only_rectangle_is_on(1, 1)

    def test_can_create_rectangle_size_1_3(self):
        self.screen.rect(1, 3)
        self.check_only_rectangle_is_on(1, 3)

    def test_can_create_rectangle_size_3_1(self):
        self.screen.rect(3, 1)
        self.check_only_rectangle_is_on(3, 1)

    def test_can_create_rectangle_size_3_7(self):
        self.screen.rect(self.num_cols, self.num_rows)
        for pixel in self.screen:
            self.assertTrue(pixel)

    def test_get_item(self):
        self.screen.rect(2, 1)
        self.checkPixel(True, 0, 0)
        self.checkPixel(True, 1, 0)
        self.checkPixel(False, 0, 1)
        self.checkPixel(False, 1, 1)
        self.checkPixel(False, 2, 0)

    def test_rotate_first_row_by_1(self):
        self.screen.rect(1, 1)
        self.screen.rotate_row(0, 1)
        self.checkPixel(False, 0, 0)
        self.checkPixel(True, 1, 0)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 0, 1)
        self.checkPixel(False, 1, 1)
        self.checkPixel(False, 2, 1)

    def test_rotate_second_row_by_1(self):
        self.screen.rect(1, 2)
        self.screen.rotate_row(1, 1)
        self.checkPixel(True, 0, 0)
        self.checkPixel(False, 1, 0)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 0, 1)
        self.checkPixel(True, 1, 1)
        self.checkPixel(False, 2, 1)

    def test_rotate_second_row_by_2(self):
        self.screen.rect(1, 2)
        self.screen.rotate_row(1, 2)
        self.checkPixel(True, 0, 0)
        self.checkPixel(False, 1, 0)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 0, 1)
        self.checkPixel(False, 1, 1)
        self.checkPixel(True, 2, 1)

    def test_rotate_row_with_overflow(self):
        self.screen.rect(1, 2)
        self.screen.rotate_row(0, self.num_cols)
        self.check_only_rectangle_is_on(1, 2)
        self.screen.rotate_row(1, self.num_cols)
        self.check_only_rectangle_is_on(1, 2)

    def test_rotate_first_column_by_1(self):
        self.screen.rect(1, 1)
        self.screen.rotate_column(0, 1)
        self.checkPixel(False, 0, 0)
        self.checkPixel(False, 1, 0)
        self.checkPixel(True, 0, 1)
        self.checkPixel(False, 1, 1)
        self.checkPixel(False, 0, 2)
        self.checkPixel(False, 1, 2)

    def test_rotate_second_column_by_1(self):
        self.screen.rect(2, 1)
        self.screen.rotate_column(1, 1)
        self.checkPixel(True, 0, 0)
        self.checkPixel(False, 1, 0)
        self.checkPixel(False, 0, 1)
        self.checkPixel(True, 1, 1)
        self.checkPixel(False, 2, 0)
        self.checkPixel(False, 2, 1)

    def test_rotate_column_with_overlap(self):
        self.screen.rect(2, 1)
        self.screen.rotate_column(0, self.num_rows)
        self.check_only_rectangle_is_on(2, 1)
        self.screen.rotate_column(1, self.num_rows)
        self.check_only_rectangle_is_on(2, 1)


class CommandRunner(object):
    def __init__(self, screen):
        self.screen = screen

    def execute_file(self, filename):
        for command_line in data_lines(2016, filename):
            self.execute_one(command_line)

    def execute_one(self, command):
        args = command.split()
        if args[0] == "rect":
            self.execute_rect(args)
        elif args[0] == "rotate":
            self.execute_rotate(args)

    def execute_rect(self, args):
        width, height = map(int, args[1].split("x"))
        self.screen.rect(width, height)

    def execute_rotate(self, args):
        index = int(args[2].split("=")[1])
        steps = int(args[4])

        if args[1] == "column":
            self.screen.rotate_column(index, steps)
        elif args[1] == "row":
            self.screen.rotate_row(index, steps)


class TestInstructionRunner(TestCase):
    def setUp(self):
        self.mock_screen = MagicMock()
        self.runner = CommandRunner(self.mock_screen)

    def test_parse_rect_instruction(self):
        self.runner.execute_one("rect 1x2")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rect.assert_called_once_with(1, 2)

        self.mock_screen.reset_mock()
        self.runner.execute_one("rect 33x42")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rect.assert_called_once_with(33, 42)

    def test_parse_rotate_row_instruction(self):
        self.runner.execute_one("rotate row x=1 by 2")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rotate_row.assert_called_once_with(1, 2)

        self.mock_screen.reset_mock()
        self.runner.execute_one("rotate row x=12 by 21")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rotate_row.assert_called_once_with(12, 21)

    def test_parse_rotate_column_instruction(self):
        self.runner.execute_one("rotate column x=1 by 2")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rotate_column.assert_called_once_with(1, 2)

        self.mock_screen.reset_mock()
        self.runner.execute_one("rotate column x=12 by 21")
        self.assertEqual(1, len(self.mock_screen.method_calls))
        self.mock_screen.rotate_column.assert_called_once_with(12, 21)


class TestCommandRunner(TestCase):
    def test_execute_commands_example(self):
        runner = CommandRunner(Screen(3, 7))
        runner.execute_one("rect 3x2")
        runner.execute_one("rotate column x=1 by 1")
        runner.execute_one("rotate row y=0 by 4")
        runner.execute_one("rotate column x=1 by 1")
        self.assertEqual(6, sum(runner.screen))

    def test_execute_commands_mine(self):
        runner = CommandRunner(Screen(6, 50))
        runner.execute_file("day_08_mine.txt")
        self.assertEqual(116, sum(runner.screen))
        expected = [
            "#  # ###   ##    ## #### #    ###   ##  #### #### ",
            "#  # #  # #  #    # #    #    #  # #  # #       # ",
            "#  # #  # #  #    # ###  #    ###  #    ###    #  ",
            "#  # ###  #  #    # #    #    #  # #    #     #   ",
            "#  # #    #  # #  # #    #    #  # #  # #    #    ",
            " ##  #     ##   ##  #    #### ###   ##  #### #### ",
        ]
        self.assertListEqual(expected, list(runner.screen.print()))
