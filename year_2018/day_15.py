import unittest
import array


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

    def test_from_lines(self):
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

    def test_raises_for_lines_of_different_lengths(self):
        cmap = CharMap()

        self.assertRaises(AssertionError, cmap.from_lines, [
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
        return rules.results


class FindAllClosestRules:
    def __init__(self, targets, allowed_values):
        self._targets = targets
        self._allowed_values = allowed_values
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

    def next_coordinates(self, from_coordinates):
        for delta in ((0, -1), (-1, 0), (1, 0), (0, 1)):
            yield self._add_coordinates(from_coordinates, delta)

    def progress_to(self, _coordinates, value):
        return value in self._allowed_values

    def _add_coordinates(self, a, b):
        return tuple(u + v for u, v in zip(a, b))


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

    def test_find_all_closest(self):
        caves = Caves([
            "#######",
            "#E#.G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])

        self.assertListEqual(
            [(2, 2), (1, 3)],
            list(caves._find_all_closest(
                from_coords=(1, 1),
                targets=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                allowed_values=["."])
            )
        )


class Caves:
    def __init__(self, initial_map):
        self._caves = CharMap()
        self._caves.from_lines(initial_map)

    def _find_all_closest(self, from_coords, targets, allowed_values):
        finder = MapExplorer(self._caves)
        rules = FindAllClosestRules(targets, allowed_values)
        closest = finder.explore(from_coords, rules)
        for result in closest:
            yield result

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
