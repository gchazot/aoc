import unittest

from aoc_utils.char_map import CharMap
from aoc_utils.data import data_text
from aoc_utils.geometry import add_coordinates
from year_2019.int_code_processor import IntCodeProcessor, instructions_day_09, InputNeeded


DIRECTIONS = [
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1),
]


class TestPaintingRobot(unittest.TestCase):
    def test_program(self):
        panels = launch_paint_robot(start_color=0)
        self.assertEqual(1564, len(panels))

    def test_program_2(self):
        panels = launch_paint_robot(start_color=1)

        offset_x = -min(x for x, y in panels.keys())
        offset_y = -min(y for x, y in panels.keys())
        offset = (offset_x, offset_y)

        width = offset_x + max(x for x, y in panels.keys()) + 1
        height = offset_y + max(y for x, y in panels.keys()) + 1

        painting = CharMap(width_height=(width, height))
        for position, color in panels.items():
            if color == 1:
                paint_position = add_coordinates(position, offset)
                painting[paint_position] = "#"

        self.assertEqual(
            [
                " ###  #### #### ###   ##  #### #### ###    ",
                " #  # #    #    #  # #  # #    #    #  #   ",
                " #  # ###  ###  #  # #    ###  ###  ###    ",
                " ###  #    #    ###  #    #    #    #  #   ",
                " # #  #    #    #    #  # #    #    #  #   ",
                " #  # #    #### #     ##  #    #### ###    ",
            ],
            list(painting.lines()),
        )


def launch_paint_robot(start_color):
    program = list(map(int, data_text(2019, "day_11_mine.txt").split(",")))
    processor = IntCodeProcessor(program, instructions_day_09, input_values=[start_color])
    panels = {}
    robot_position = 0, 0
    robot_direction = 1
    while True:
        try:
            processor.execute()
        except InputNeeded:
            pass
        else:
            break
        finally:
            paint = processor.output_values.pop(0)
            turn = processor.output_values.pop(0)

            panels[robot_position] = paint

            robot_direction += 1 if turn == 1 else -1
            robot_direction %= len(DIRECTIONS)

            robot_position = add_coordinates(robot_position, DIRECTIONS[robot_direction])

            processor.input_values.append(panels.get(robot_position, 0))
    return panels
