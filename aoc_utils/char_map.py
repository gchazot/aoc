import array
import unittest


class TestCharMap(unittest.TestCase):
    def test_starts_empty(self):
        cmap = CharMap()
        self.assertEqual(0, len(cmap))
        self.assertEqual(0, cmap.width)
        self.assertEqual(0, cmap.height)

    def test_create_with_dimensions(self):
        width = 4
        height = 3
        cmap = CharMap(width_height=(width, height))

        self.assertEqual(width, cmap.width)
        self.assertEqual(height, cmap.height)
        for x in range(width):
            for y in range(height):
                self.assertEqual(None, cmap[x, y])

    def test_create_from_lines(self):
        cmap = CharMap(input_lines=[
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

    def test_raises_for_lines_of_different_lengths(self):
        self.assertRaises(AssertionError, CharMap, input_lines=[
            "abcd",
            "ab"
        ])

    def test_contains(self):
        width = 4
        height = 3
        cmap = CharMap(width_height=(width, height))

        for x in range(-1, width + 1):
            for y in range(-1, height + 1):
                if 0 <= x < width and 0 <= y < height:
                    self.assertTrue((x, y) in cmap)
                else:
                    self.assertFalse((x, y) in cmap)

    def test_get_raises_for_index_out_of_range(self):
        cmap = CharMap(input_lines=[
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
        cmap = CharMap(input_lines=[
            "abcd",
            "efgh",
            "ijkl",
        ])

        cmap[1, 1] = "z"
        self.assertEqual("z", cmap[1, 1])

    def test_iterate_coordinates_in_reading_order(self):
        cmap = CharMap(input_lines=[
            "abc",
            "def",
        ])
        self.assertListEqual([
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
        ], list(cmap.coordinates()))

    def test_iterate_values_in_reading_order(self):
        cmap = CharMap(input_lines=[
            "abc",
            "def",
        ])
        self.assertListEqual([
            "a", "b", "c",
            "d", "e", "f",
        ], list(cmap.values()))

    def test_iterate_items_in_reading_order(self):
        cmap = CharMap(input_lines=[
            "abc",
            "def",
        ])
        self.assertListEqual([
            ((0, 0), "a"), ((1, 0), "b"), ((2, 0), "c"),
            ((0, 1), "d"), ((1, 1), "e"), ((2, 1), "f"),
        ], list(cmap.items()))


class CharMap:
    def __init__(self, input_lines=None, width_height=None):
        self.width = 0
        self.height = 0
        self._data = array.array('B')
        self._codes = {None: 0}

        if input_lines is not None:
            self._init_from_lines(input_lines)
        elif width_height is not None:
            self._init_from_dimensions(*width_height)

    def _init_from_dimensions(self, width, height):
        self.width = width
        self.height = height
        self._data = array.array('B', (0 for _ in range(self.width * self.height)))

    def _init_from_lines(self, input_lines):
        widths = []
        for line in input_lines:
            widths.append(len(line))
            self._data.extend(self._code(c) for c in line)

        assert len(set(widths)) == 1
        self.width = widths[0]
        self.height = len(widths)

    def __contains__(self, coordinates):
        try:
            self._get_offset(coordinates)
        except IndexError:
            return False
        else:
            return True

    def __getitem__(self, coordinates):
        offset = self._get_offset(coordinates)
        code = self._data[offset]
        return self._value(code)

    def __setitem__(self, coordinates, value):
        offset = self._get_offset(coordinates)
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

    def _get_offset(self, coordinates):
        x, y = coordinates
        if x < 0 or x >= self.width:
            raise IndexError("x={} out of range".format(x))
        if y < 0 or y >= self.height:
            raise IndexError("x={} out of range".format(y))
        offset = self.width * y + x
        return offset

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


class MapExplorer:
    def __init__(self, char_map):
        self._map = char_map
        self._distances = CharMap(width_height=(char_map.width, char_map.height))

    def explore(self, starting_point, rules):
        progress_points = [starting_point]

        steps = 0
        while len(progress_points) > 0 and not rules.stop_progressing():
            steps += 1
            new_progress_points = []
            for coordinates in filter(rules.examine, progress_points):
                self._distances[coordinates] = steps
                for next_coordinates in rules.next_coordinates(coordinates):
                    if self._distances[next_coordinates] is None:
                        value = self._map[next_coordinates]
                        if rules.progress_to(next_coordinates, value):
                            new_progress_points.append(next_coordinates)
            progress_points = new_progress_points
        return


class ProgressRules(object):
    def __init__(self, allowed_values):
        self._allowed_values = allowed_values

    def stop_progressing(self):
        """
        Decide whether to continue with the next iteration of progress
        :return: True to stop, False to continue
        """
        return False

    def examine(self, _coordinates):
        """
        Examine some coordinates and decide whether to continue progressing from those
        :param _coordinates: The coordinates to examine
        :return: True to continue progressing from coordinates, False otherwise
        """
        return True

    def next_coordinates(self, from_coordinates):
        """
        Generate the next possible coordinates to progress
        :param from_coordinates: The coordinates to progress from
        :return: a generator of the next coordinates
        """
        for delta in ((0, -1), (-1, 0), (1, 0), (0, 1)):
            yield self.add_coordinates(from_coordinates, delta)

    def progress_to(self, _coordinates, value):
        """
        Check whether it is allowed to progress to some coordinates with a given value
        :param _coordinates: The coordinates to check
        :param value: The value at the coordinates
        :return: True iff progress is allowed
        """
        return value in self._allowed_values

    @staticmethod
    def add_coordinates(a, b):
        """Helper to add 2 coordinates vectors"""
        return tuple(u + v for u, v in zip(a, b))
