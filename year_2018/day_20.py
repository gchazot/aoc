from __future__ import print_function
from collections import defaultdict
import unittest
from aoc_utils.data import data_text
from aoc_utils.char_map import add_coordinates, CharMap


class TestRouteParser(unittest.TestCase):
    def test_simple_route(self):
        example = '^WNE$'
        route = RouteItem(example)

        self.assertEqual(1, len(route.items))
        self.assertEqual(example[1:-1], route.items[0])

    def test_simple_option(self):
        route = RouteItem('^(WNE|EWN)$')

        self.assertEqual(1, len(route.items))
        self.assertEqual(2, len(route.items[0].items))
        self.assertEqual('WNE', route.items[0].items[0])
        self.assertEqual('EWN', route.items[0].items[1])

    def test_empty_options(self):
        route_a = RouteItem('^(WNE|)$')

        self.assertEqual(1, len(route_a.items))
        self.assertEqual(2, len(route_a.items[0].items))
        self.assertEqual('WNE', route_a.items[0].items[0])
        self.assertEqual('', route_a.items[0].items[1])

        route_b = RouteItem('^(|EWN)$')

        self.assertEqual(1, len(route_b.items))
        self.assertEqual(2, len(route_b.items[0].items))
        self.assertEqual('', route_b.items[0].items[0])
        self.assertEqual('EWN', route_b.items[0].items[1])

    def test_nested_option(self):
        route = RouteItem('^AB(WNE|CD(123|EWN)EF)GH$')
        '''
        AB
        (
            WNE
            |
            CD
            (
                123
                |
                EWN
            )
            EF
        )
        GH
        '''

        self.assertEqual(3, len(route.items))
        self.assertFalse(route.is_choice)
        self.assertEqual('AB', route.items[0])
        self.assertEqual('GH', route.items[2])

        self.assertTrue(route.items[1].is_choice)
        self.assertEqual(2, len(route.items[1].items))
        self.assertEqual('WNE', route.items[1].items[0])

        self.assertFalse(route.items[1].items[1].is_choice)
        self.assertEqual(3, len(route.items[1].items[1].items))
        self.assertEqual('CD', route.items[1].items[1].items[0])
        self.assertEqual('EF', route.items[1].items[1].items[2])

        self.assertTrue(route.items[1].items[1].items[1].is_choice)
        self.assertEqual('123', route.items[1].items[1].items[1].items[0])
        self.assertEqual('EWN', route.items[1].items[1].items[1].items[1])

    def test_parse_mine(self):
        route_str = data_text(2018, "day_20_mine.txt")
        route = RouteItem(route_str)
        self.assertFalse(route.is_choice)
        self.assertEqual(2, len(route.items))
        self.assertTrue(route.items[1].is_choice)

    def test_make_network(self):
        route = RouteItem('^NN(WNE|SS(EEE|EWN)WW)SS$')

        start_coords = (0, 0)
        network = defaultdict(set)
        ends = route.make_network([start_coords], network)

        self.assertSetEqual({(1, 2), (0, -1), (-2, 1)}, ends)

    def test_furthest_distance(self):
        self.assertEqual(3, furthest_distance('^WNE$'))
        self.assertEqual(10, furthest_distance('^ENWWW(NEEE|SSE(EE|N))$'))
        self.assertEqual(18, furthest_distance('^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$'))
        self.assertEqual(18, furthest_distance('^ENNWSWW(|NEWS)(WNSE|)SSSEENEE(SWEN|)NNN$'))
        self.assertEqual(23, furthest_distance(
            '^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$'
        ))
        self.assertEqual(31, furthest_distance(
            '^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$'
        ))
        self.assertEqual(2, furthest_distance('^WNES$'))
        self.assertEqual(4, furthest_distance('^WW((NNEE))SS(NE|)$'))
        self.assertEqual(6, furthest_distance('^WW(SSWWNNNNEEEE(|SS)|WW)$'))

    def test_furthest_distance_mine(self):
        route_str = data_text(2018, "day_20_mine.txt")
        distance = furthest_distance(route_str)
        self.assertEqual(4239, distance)

    def test_far_distances_mine(self):
        route_str = data_text(2018, "day_20_mine.txt")
        distances = path_distances(route_str)
        long_distances = [d for d in distances.values() if d >= 1000]
        self.assertEqual(8205, len(long_distances))

    @unittest.skip("Prints to stdout")
    def test_map(self):
        route = RouteItem('^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$')
        route.mapit()

    @unittest.skip("Prints to stdout")
    def test_map_mine(self):
        route_str = data_text(2018, "day_20_mine.txt")
        route = RouteItem(route_str)
        print(route.max_length())
        route.mapit()


class RouteItem:
    def __init__(self, route_regex=None, is_choice=False):
        self.items = []
        self.is_choice = is_choice
        if route_regex is not None:
            if route_regex[0] == '^':
                assert route_regex[-1] == '$'
                route_regex = route_regex[1:-1]
            self.parse(route_regex)

    def parse(self, route_regex):
        self._parse_split_groups(route_regex)
        self._parse_split_options()

    def _parse_split_groups(self, route_regex):
        stack = [self]
        start = 0
        i = 0
        while i < len(route_regex):
            value = route_regex[i]
            if value in ['(', '^']:
                stack[-1].append_non_empty(route_regex[start:i])
                child = RouteItem()
                stack[-1].append(child)
                stack.append(child)
                start = i + 1
            elif value in [')', '$']:
                stack[-1].append(route_regex[start:i])
                stack.pop()
                start = i + 1
            i += 1
        self.append_non_empty(route_regex[start:i])

    def _parse_split_options(self):
        new_items = [[]]
        for i, item in enumerate(self.items):
            if isinstance(item, RouteItem):
                item._parse_split_options()
                new_items[-1].append(item)
            elif item.count('|') > 0:
                splits = item.split('|')

                new_items[-1].append(splits[0])
                new_items.extend(splits[1:-1])
                new_items.append([splits[-1]])
            else:
                new_items[-1].append(item)

        if len(new_items) > 1:
            self.items = []
            self.is_choice = True
            for new_item_items in new_items:
                if len(new_item_items) > 1:
                    new_item = RouteItem()
                    new_item.items = new_item_items
                else:
                    new_item = new_item_items[0]

                self.items.append(new_item)

    def append(self, route_slice):
        self.items.append(route_slice)

    def append_non_empty(self, route_slice):
        if len(route_slice) > 0:
            self.items.append(route_slice)

    def __repr__(self):
        children = map(str, self.items)
        if self.is_choice:
            return '(' + '|'.join(children) + ')'
        else:
            return '[' + ''.join(children) + ']'

    def max_depth(self):
        result = 0
        for child in self.items:
            if isinstance(child, RouteItem):
                child_max = child.max_depth() + 1
            else:
                child_max = 1
            result = max(result, child_max)
        return result

    def max_length(self):
        result = 0
        for item in self.items:
            if isinstance(item, RouteItem):
                item_length = item.max_length()
            else:
                item_length = len(item)

            if self.is_choice:
                result = max(result, item_length)
            else:
                result += item_length

        return result

    def make_network(self, start_coords, network):
        end_coords = set()
        if self.is_choice:
            for start in start_coords:
                for choice in self.items:
                    if isinstance(choice, RouteItem):
                        ends = choice.make_network([start], network)
                        end_coords.update(ends)
                    else:
                        end = traverse_string(choice, start, network)
                        end_coords.add(end)
        else:
            for element in self.items:
                end_coords.clear()
                for start in start_coords:
                    if isinstance(element, RouteItem):
                        ends = element.make_network([start], network)
                        end_coords.update(ends)
                    else:
                        end = traverse_string(element, start, network)
                        end_coords.add(end)
                start_coords = end_coords.copy()
        return end_coords

    def mapit(self):
        start_coords = (0, 0)
        network = defaultdict(set)
        self.make_network([start_coords], network)

        xs = {coords[0] for coords in network.keys()}
        ys = {coords[1] for coords in network.keys()}

        offset_x = -min(xs)
        offset_y = -min(ys)
        dim_x = max(xs) + 1
        dim_y = max(ys) + 1

        def convert(x, y):
            return (x + offset_x)*2 + 1, (y + offset_y)*2 + 1

        castle = CharMap(width_height=convert(dim_x, dim_y))
        for current, options in network.items():
            castle[convert(*current)] = '.' if current != start_coords else 'X'
            for destination in options:
                direction = (
                    destination[0] - current[0],
                    destination[1] - current[1],
                )
                if direction[0] != 0:
                    display = '|'
                else:
                    display = '-'
                interface = add_coordinates(convert(*current), direction)
                castle[interface] = display

        castle.echo()


DIRECTIONS = {
    'N': (0, -1),
    'S': (0, 1),
    'E': (1, 0),
    'W': (-1, 0),
}


def traverse_string(direction_string, start, network):
    current = start
    for direction in direction_string:
        step = DIRECTIONS[direction]
        following = add_coordinates(current, step)

        network[current].add(following)
        network[following].add(current)

        current = following
    return current


def network_distances(network, start):
    progress_points = {start}
    distances = {}

    distance = 0
    while progress_points:
        next_progress_points = set()
        for current in progress_points:
            if current not in distances:
                distances[current] = distance
                next_progress_points.update(network[current])
        progress_points = next_progress_points
        distance += 1
    return distances


def furthest_distance(route_regex):
    distances = path_distances(route_regex)
    return max(distances.values())


def path_distances(route_regex):
    route = RouteItem(route_regex)
    start_coords = (0, 0)
    network = defaultdict(set)
    route.make_network([start_coords], network)
    distances = network_distances(network, start_coords)
    return distances
