import unittest
import array


class TestCharMap(unittest.TestCase):
    def test_starts_empty(self):
        cmap = CharMap()
        self.assertEqual(0, len(cmap))

    def test_raises_for_lines_of_different_lengths(self):
        cmap = CharMap()

        self.assertRaises(AssertionError, cmap.from_lines, [
            "abcd",
            "ab"
        ])

    def test_set_and_access_data(self):
        cmap = CharMap()
        cmap.from_lines([
            "abcd",
            "efgh",
            "ijkl",
        ])
        self.assertEqual(4, cmap.width)
        self.assertEqual(3, cmap.height)
        self.assertEqual('a', cmap[0, 0])
        self.assertEqual('b', cmap[1, 0])
        self.assertEqual('k', cmap[2, 2])
        self.assertEqual('l', cmap[3, 2])

    def test_get_raises_for_index_out_of_range(self):
        cmap = CharMap()
        cmap.from_lines([
            "abcd",
            "efgh",
            "ijkl",
        ])

        self.assertRaises(IndexError, cmap.__getitem__, [-1, 0])
        self.assertRaises(IndexError, cmap.__getitem__, [0, -1])
        self.assertRaises(IndexError, cmap.__getitem__, [-1, -1])
        self.assertRaises(IndexError, cmap.__getitem__, [4, 0])
        self.assertRaises(IndexError, cmap.__getitem__, [0, 3])
        self.assertRaises(IndexError, cmap.__getitem__, [4, 3])

    def test_set_item_value(self):
        cmap = CharMap()
        cmap.from_lines([
            "abcd",
            "efgh",
            "ijkl",
        ])

        cmap[1, 1] = "z"
        self.assertEqual("z", cmap[1, 1])

    def test_iterate_coordinates_in_reading_order(self):
        cmap = CharMap()
        cmap.from_lines([
            "abc",
            "def",
        ])
        self.assertListEqual([
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
        ], list(cmap.coordinates()))

    def test_iterate_values_in_reading_order(self):
        cmap = CharMap()
        cmap.from_lines([
            "abc",
            "def",
        ])
        self.assertListEqual([
            "a", "b", "c",
            "d", "e", "f",
        ], list(cmap.values()))

    def test_iterate_items_in_reading_order(self):
        cmap = CharMap()
        cmap.from_lines([
            "abc",
            "def",
        ])
        self.assertListEqual([
            ((0, 0), "a"), ((1, 0), "b"), ((2, 0), "c"),
            ((0, 1), "d"), ((1, 1), "e"), ((2, 1), "f"),
        ], list(cmap.items()))

    def test_find_all_closest(self):
        cmap = CharMap([
            "#######",
            "#E#.G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        self.assertListEqual(
            [(2, 2), (1, 3)],
            list(cmap.find_all_closest(
                from_coords=(1, 1),
                target_list=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                allowed_values=["."]
            ))
        )


class CharMap:
    def __init__(self, input_lines=None, width_height=None):
        self.width = 0
        self.height = 0
        self._data = array.array('B')
        self._codes = {None: 0}
        if input_lines is not None:
            self.from_lines(input_lines)
        elif width_height is not None:
            self.width, self.height = width_height
            self._data = array.array('B', (0 for _ in range(self.width * self.height)))

    def from_lines(self, input_lines):
        widths = []
        for line in input_lines:
            widths.append(len(line))
            self._data.extend(self._code(c) for c in line)

        assert len(set(widths)) == 1
        self.width = widths[0]
        self.height = len(widths)

    def __getitem__(self, coordinates):
        offset = self._get_offset(*coordinates)
        code = self._data[offset]
        return self._value(code)

    def __setitem__(self, coordinates, value):
        offset = self._get_offset(*coordinates)
        code = self._code(value)
        self._data[offset] = code

    def __len__(self):
        return len(self._data)

    def coordinates(self):
        for y in range(self.height):
            for x in range(self.width):
                yield x, y

    def values(self):
        for code in self._data:
            yield self._value(code)

    def items(self):
        for cordinates in self.coordinates():
            yield cordinates, self[cordinates]

    def _get_offset(self, x, y):
        if x < 0 or x >= self.width:
            raise IndexError("x={} out of range".format(x))
        if y < 0 or y >= self.height:
            raise IndexError("x={} out of range".format(y))
        offset = self.width * y + x
        return offset

    def find_all_closest(self, from_coords, target_list, allowed_values):
        closest = ClosestFinder(self, allowed_values).find_all_closest(from_coords, target_list)
        for result in closest:
            yield result

    def _code(self, value):
        try:
            return self._codes[value]
        except KeyError:
            next_code = len(self._codes)
            self._codes[value] = next_code
            return next_code

    def _value(self, code):
        for item_value, item_code in self._codes.items():
            if item_code == code:
                return item_value
        raise ValueError("Code {} not found".format(code))


class ClosestFinder:
    def __init__(self, char_map, allowed_values):
        self._map = char_map
        self._allowed_values = allowed_values
        self._distances = CharMap(width_height=(char_map.width, char_map.height))

    def find_all_closest(self, start_coordinates, targets):
        steps = 0
        progress_points = [start_coordinates]
        found_one = False

        while not found_one and len(progress_points) > 0:
            steps += 1
            new_progress_points = []
            for coordinates in progress_points:
                if coordinates in targets:
                    yield coordinates
                    found_one = True
                    continue
                for next_coordinates, value in self._next_items(coordinates):
                    if value in self._allowed_values and self._distances[next_coordinates] is None:
                        self._distances[next_coordinates] = steps
                        new_progress_points.append(next_coordinates)
            progress_points = new_progress_points

    def _next_items(self, from_coordinates):
        for i, j in ((0, -1), (-1, 0), (1, 0), (0, 1)):
            try:
                next_coordinates = from_coordinates[0] + i, from_coordinates[1] + j
                value = self._map[next_coordinates]
                yield next_coordinates, value
            except IndexError:
                continue


class TestCaves(unittest.TestCase):
    def test_get_targets(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual([(4, 1), (2, 3), (5, 3)], list(caves.get_targets("E")))
        self.assertListEqual([(1, 1)], list(caves.get_targets("G")))

    def test_get_in_range(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual([(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                             list(caves.get_in_range("E")))


class Caves:
    def __init__(self, initial_map):
        self._caves = CharMap()
        self._caves.from_lines(initial_map)

    def get_in_range(self, opponent):
        in_range = []
        for x, y in self.get_targets(opponent):
            for i, j in ((0, -1), (-1, 0), (1, 0), (0, 1)):
                try:
                    u = x + i
                    v = y + j
                    value = self._caves[u, v]
                except IndexError:
                    continue
                else:
                    if value == '.':
                        in_range.append((u, v))
        return sorted(in_range, key=lambda tup: (tup[1], tup[0]))

    def get_targets(self, opponent):
        for cordinates, entry in self._caves.items():
            if entry not in ['#', '.', opponent]:
                yield cordinates
