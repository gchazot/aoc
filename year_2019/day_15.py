import unittest

from aoc_utils.char_map import CharMap, MapExplorer, ProgressRules
from aoc_utils.geometry import add_coordinates, manhattan_distance
from year_2019.int_code_processor import IntCodeProcessor, instructions_day_09, InputNeeded, read_program

WALL = "#"
EMPTY = "."
DROID = "D"
OXYGEN = "O"
UNKNOWN = None

NORTH = 1
SOUTH = 2
WEST = 3
EAST = 4

OPPOSITE = {
    1: 2,
    2: 1,
    3: 4,
    4: 3,
}

STEPS = {
    1: (0, -1),
    2: (0, 1),
    3: (-1, 0),
    4: (1, 0),
}


class TestDroidController(unittest.TestCase):
    def test_initialise(self):
        droid = DroidController(program=[])

        self.assertEqual((0, 0), droid.position)
        self.assertEqual({(0, 0): EMPTY}, droid.ship)
        self.assertEqual(["D"], list(droid.render()))

    def test_move(self):
        program_file = "day_15_mine.txt"
        interface_program = read_program(program_file)
        droid = DroidController(program=interface_program)

        self.assertFalse(droid.move(EAST))
        self.assertFalse(droid.move(NORTH))
        self.assertTrue(droid.move(WEST))
        self.assertFalse(droid.move(SOUTH))
        self.assertTrue(droid.move(EAST))
        self.assertFalse(droid.move(NORTH))

    @unittest.skip("Too slow")
    def test_explore(self):
        program_file = "day_15_mine.txt"
        interface_program = read_program(program_file)
        droid = DroidController(program=interface_program)

        self.assertTrue(droid.find(OXYGEN))
        self.assertEqual(212, len(droid.path()) - 1)

        self.assertFalse(droid.find(UNKNOWN))
        self.assertEqual(358, droid.fill_time())


class DroidController:
    def __init__(self, program):
        self.position = (0, 0)
        self.ship = {self.position: EMPTY}
        self.interface = IntCodeProcessor(program, instructions_day_09)

    def find(self, what):
        while self.ship[self.position] != what:
            for direction in NORTH, SOUTH, WEST, EAST:
                next_position = add_coordinates(self.position, STEPS[direction])
                if next_position not in self.ship:
                    self.move(direction)
                    break
            else:
                ship_map = self.map()

                to_explore = set()
                for coordinates, state in ship_map.items():
                    if state not in (DROID, EMPTY):
                        continue
                    for direction in NORTH, SOUTH, WEST, EAST:
                        neighbour = add_coordinates(coordinates, STEPS[direction])
                        if neighbour not in ship_map or ship_map[neighbour] is UNKNOWN:
                            to_explore.add(coordinates)
                            break

                if not to_explore:
                    return False

                path_finder = MapExplorer(ship_map)
                rules = FindAllClosestRules(allowed_values=[EMPTY], targets=to_explore)
                droid_location = list(ship_map.search(DROID))[0]
                path_finder.explore(start_point=droid_location, rules=rules)

                destinations = rules.results
                rules = FindAllClosestRules(allowed_values=[EMPTY], targets=destinations)
                path = path_finder.shortest_path(droid_location, destinations[0], rules)
                for destination in path[1:]:
                    for direction in NORTH, SOUTH, WEST, EAST:
                        next_location = add_coordinates(droid_location, STEPS[direction])
                        if next_location == destination:
                            self.move(direction)
                            droid_location = next_location
                            break
        return True

    def path(self):
        ship_map = self.map()
        path_finder = MapExplorer(ship_map)

        map_position = list(ship_map.search(DROID))[0]

        map_origin = (map_position[0] - self.position[0], map_position[1] - self.position[1])

        rules = FindAllClosestRules(allowed_values=[EMPTY, DROID, OXYGEN], targets=[map_position])
        path_finder.explore(start_point=map_origin, rules=rules)
        path = path_finder.shortest_path(map_origin, map_position, rules)

        return path

    def fill_time(self):
        ship_map = self.map()
        path_finder = MapExplorer(ship_map)

        map_position = list(ship_map.search(OXYGEN))[0]

        rules = ProgressRules(allowed_values=[EMPTY, DROID])
        path_finder.explore(start_point=map_position, rules=rules)
        result = path_finder.furthest_point(map_position, rules)
        return result[1] - 1

    def move(self, direction):
        self.interface.input_values.append(direction)
        try:
            self.interface.execute()
        except InputNeeded:
            pass

        response = self.interface.output_values.pop(0)
        destination = add_coordinates(self.position, STEPS[direction])

        if response == 0:
            self.ship[destination] = WALL
            return False
        else:
            self.position = destination
            if response == 1:
                self.ship[destination] = EMPTY
            elif response == 2:
                self.ship[destination] = OXYGEN
            else:
                raise RuntimeError("Unknown response {}".format(response))
            return True

    def render(self):
        ship_map = self.map()
        return ship_map.lines()

    def echo(self):
        rendered = self.map()
        rendered.echo()

    def map(self):
        min_x = min(x for x, y in self.ship.keys())
        max_x = max(x for x, y in self.ship.keys())
        min_y = min(y for x, y in self.ship.keys())
        max_y = max(y for x, y in self.ship.keys())

        width = max_x - min_x + 1
        height = max_y - min_y + 1

        ship = CharMap(width_height=(width, height), typecode='H')
        for coordinates, value in self.ship.items():
            location = add_coordinates(coordinates, (-min_x, -min_y))
            ship[location] = value

        location = add_coordinates(self.position, (-min_x, -min_y))
        ship[location] = DROID

        return ship


class FindAllClosestRules(ProgressRules):
    def __init__(self, targets, allowed_values):
        super(FindAllClosestRules, self).__init__(allowed_values)
        self._targets = targets
        self._found_one = False
        self.results = []

    def stop_progressing(self):
        return self._found_one

    def examine(self, coordinates):
        if coordinates in self._targets:
            self._found_one = True
            self.results.append(coordinates)
            return False
        return True

    def solve_tie(self, coordinate_options, start_point, end_point):
        return min(
            coordinate_options,
            key=lambda x: manhattan_distance(x, start_point) + manhattan_distance(x, end_point),
        )
