import unittest
from aoc_utils.char_map import CharMap, add_coordinates
from aoc_utils.data import data_lines


class TestWaterCaves(unittest.TestCase):
    @staticmethod
    def make_water_caves():
        caves = WaterCaves([
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ])
        return caves

    def test_init(self):
        caves = self.make_water_caves()

        self.assertEqual(1, len(list(caves.sources.all())))
        self.assertEqual(14, caves.width)
        self.assertEqual(14, caves.height)

    def test_fill_with_water(self):
        caves = self.make_water_caves()

        caves.fill_with_water()
        self.assertEqual(57, caves.count_water())

        caves.clear_draining_water()
        self.assertEqual(29, caves.count_water())

    def test_mine(self):
        lines = data_lines(2018, "day_17_mine.txt")              
        caves = WaterCaves(lines)

        caves.fill_with_water()
        self.assertEqual(31667, caves.count_water())

        caves.clear_draining_water()
        self.assertEqual(25018, caves.count_water())


class WaterCaves(CharMap):
    def __init__(self, input_lines):
        self.sources = SourceManager((500, 0), DOWN)
        self.bottoms = []
        self.walls = []

        lines = WaterCavesLines(input_lines)

        offset_x, offset_y = lines.offsets(self.sources)
        self.offsets = -offset_x, -offset_y
        self.starting_depth = lines.offsets(sources=None)[1]

        width_height = lines.width_height(self.sources)
        super(WaterCaves, self).__init__(width_height=width_height)

        lines.set_char_map(self, self.sources)

    def fill_with_water(self):
        counter = 0
        while self.sources.any_active():
            counter += 1
            if counter > 100000:
                raise RuntimeError("Too many loops")

            active_sources = list(self.sources.active())
            for source in active_sources:
                next_coordinates = source.next_coords()

                try:
                    next_value = self.get(next_coordinates)
                except IndexError:
                    source.dead_end()
                    continue

                if source.direction == DOWN:
                    if next_value in [None, '~']:
                        self.set(next_coordinates, "|")
                        source.progress()
                    elif next_value in ['#', '.']:
                        source.bottom()
                    elif next_value in ['+', '|', '@']:
                        source.cancel()
                elif source.direction in [LEFT, RIGHT]:
                    below = progress_to(source.coordinates, DOWN)
                    below_value = self.get(below)
                    if below_value is None:
                        source.overflow()
                        self.set(source.coordinates, '+')
                    else:
                        if next_value is None:
                            self.set(next_coordinates, '~')
                            source.progress()
                        elif next_value in ['+', '@']:
                            source.progress()
                            self.merge_overflows(source)
                        elif next_value in ['~', '.', '|']:
                            source.progress()
                        elif next_value == '#':
                            self.dead_end(source)

    def clear_draining_water(self):
        for source in self.sources.all():
            if source.is_overflowing:
                here = source.coordinates
                direction = UP - source.direction
                try:
                    while self.get(here) not in [None, '#']:
                        self.set(here, '*')
                        here = progress_to(here, direction)
                except IndexError:
                    continue

        for coordinates, value in self.items():
            if value not in ['#', '~', '.', '@']:
                self[coordinates] = ' '

    def count_water(self):
        countable_values = ['+', '|', '~', '.', '@']
        to_ignore = 0
        for x in range(self.width):
            for y in range(self.starting_depth):
                if self[x, y] in countable_values:
                    to_ignore += 1
        return self.count_values(countable_values) - to_ignore

    def dead_end(self, source):
        source.dead_end()

        for parent_source in source.parents:
            siblings = parent_source.children
            if any(sibling.is_active() or sibling.is_overflowing for sibling in siblings):
                return

            self.set(parent_source.coordinates, '.')
            parent_source.de_bottom()

            self.resume_overflow(parent_source)

    def merge_overflows(self, source):
        duplicates = [
            s for s in self.sources.all() if (
                    s != source
                    and s.coordinates == source.coordinates
                    and s.direction == source.direction
                    and not s.is_cancelled)
        ]
        if len(duplicates) == 1:
            source.cancel()
            for parent_source in source.parents:
                parent_source.children.append(duplicates[0])
                duplicates[0].parents.append(parent_source)

    def resume_overflow(self, source):
        for parent_source in source.parents:
            if source.coordinates != parent_source.coordinates:
                continue
            source.dead_end()
            parent_source.de_overflow()
            self.set(parent_source.coordinates, '@')

    def get(self, coordinates):
        return self[self.offset(coordinates)]

    def set(self, coordinates, value):
        offset_coordinates = self.offset(coordinates)

        if value != '#' and self[offset_coordinates] == "#":
            self.echo()
            raise RuntimeError("Overwriting wall", offset_coordinates, value)

        self[offset_coordinates] = value

    def offset(self, coordinates):
        return add_coordinates(coordinates, self.offsets)


class WaterCavesLines:
    def __init__(self, input_lines):
        self.bottoms = []
        self.walls = []

        self.add_lines(input_lines)

    def add_lines(self, lines):
        for line in lines:
            self.add_line(line.strip())

    def add_line(self, line):
        parts = line.split(", ")
        fields = [part.split("=") for part in parts]
        where = int(fields[0][1])
        span = list(map(int, fields[1][1].split("..")))
        if fields[0][0] == "x":
            self.walls.append((where, span))
        else:
            self.bottoms.append((where, span))

    def width_height(self, sources):
        offset_x, max_x, offset_y, max_y = self.dimensions(sources)
        width = max_x - offset_x + 1
        height = max_y - offset_y + 1

        return width, height

    def offsets(self, sources):
        offset_x, max_x, offset_y, max_y = self.dimensions(sources)
        return offset_x, offset_y

    def dimensions(self, sources):
        xs = (
            [bottom[1][0] for bottom in self.bottoms] +
            [bottom[1][1] for bottom in self.bottoms] +
            [wall[0] for wall in self.walls]
        )
        ys = (
            [bottom[0] for bottom in self.bottoms] +
            [wall[1][0] for wall in self.walls] +
            [wall[1][1] for wall in self.walls]
        )

        if sources:
            xs += [source[0] for source in sources.all()]
            ys += [source[1] for source in sources.all()]

        left_margin = 2
        return min(xs) - left_margin, max(xs), min(ys), max(ys)

    def set_char_map(self, char_map, sources):
        def add_points(char, points):
            for point in points:
                char_map.set((point[0], point[1]), char)

        def add_verticals(char, verticals):
            for vertical in verticals:
                x = vertical[0]
                ys = range(vertical[1][0], vertical[1][1] + 1)
                add_points(char, ((x, y) for y in ys))

        def add_horizontals(char, horizontals):
            for horizontal in horizontals:
                y = horizontal[0]
                xs = range(horizontal[1][0], horizontal[1][1] + 1)
                add_points(char, ((x, y) for x in xs))

        add_points('+', sources.all())
        add_verticals("#", self.walls)
        add_horizontals("#", self.bottoms)


DOWN, LEFT, RIGHT, UP = range(4)
PROGRESS_STEPS = {
    DOWN: (0, 1),
    LEFT: (-1, 0),
    RIGHT: (1, 0),
    UP: (0, -1),
}


def progress_to(coordinates, direction):
    progress = PROGRESS_STEPS[direction]
    return add_coordinates(coordinates, progress)


class SourceManager:
    def __init__(self, coordinates, direction):
        self._all = []
        self._active = set()
        self.root = Source(coordinates, direction, self)

    def all(self):
        return self._all

    def active(self):
        return (self._all[index] for index in self._active)

    def any_active(self):
        return len(self._active) > 0

    def add(self, source):
        if source.is_active():
            self._active.add(len(self._all))
        self._all.append(source)

    def update(self, source):
        index = self._all.index(source)
        if source.is_active():
            self._active.add(index)
        else:
            self._active.discard(index)


class Source:
    def __init__(self, coordinates, direction, manager):
        self.coordinates = coordinates
        self.direction = direction
        self.parents = []
        self.manager = manager
        self.is_bottom = False
        self.is_dead_end = False
        self.is_overflowing = False
        self.is_cancelled = False
        self.children = []

        self.manager.add(self)

    def __getitem__(self, item):
        return self.coordinates[item]

    def __iter__(self):
        return self.coordinates

    def progress(self, direction=None):
        self.coordinates = self.next_coords(direction)

    def next_coords(self, direction=None):
        actual_direction = self.direction if direction is None else direction
        return progress_to(self.coordinates, actual_direction)

    def update(self):
        self.manager.update(self)

    def bottom(self):
        self.is_bottom = True
        self.update()
        self.add_child(self.coordinates, LEFT)
        self.add_child(self.coordinates, RIGHT)

    def de_bottom(self):
        self.progress(UP)
        self.is_bottom = False
        self.update()

    def dead_end(self):
        self.is_dead_end = True
        self.update()

    def overflow(self):
        self.is_overflowing = True
        self.add_child(self.coordinates, DOWN)
        self.update()

    def de_overflow(self):
        self.is_overflowing = False
        self.update()

    def cancel(self):
        self.is_cancelled = True
        self.update()

    def is_active(self):
        return not (self.is_bottom or self.is_dead_end or self.is_overflowing or self.is_cancelled)

    def add_child(self, coordinates, direction):
        child = Source(coordinates, direction, self.manager)
        child.parents.append(self)
        self.children.append(child)

    def __repr__(self):
        return str((
            self.coordinates,
            self.is_bottom,
            self.is_dead_end,
            self.is_overflowing,
            self.is_cancelled,
            self.direction,
            len(self.children),
        ))
