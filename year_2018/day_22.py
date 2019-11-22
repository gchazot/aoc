from collections import defaultdict
import unittest


class TestCavesAgain(unittest.TestCase):
    def test_example(self):
        caves = CavesAgain(depth=510, target=(10, 10))
        self.assertEqual(114, caves.calculate_total_risk())
        self.assertEqual(45, caves.shortest_path())

    def test_mine(self):
        caves = CavesAgain(depth=4848, target=(15, 700))
        self.assertEqual(11359, caves.calculate_total_risk())
        self.assertEqual(976, caves.shortest_path())


ROCKY = NOTHING = 0
WET = TORCH = 1
NARROW = CLIMB = 2


class CavesAgain:
    def __init__(self, depth, target):
        self.depth = depth
        self.target = target
        self.max_coordinates = 3 * target[0], 3 * target[1]
        self.erosions = {
            (0, 0): 0,
            target: 0,
        }
        self._calculate_erosions()

    def calculate_total_risk(self):
        return sum(self._get_risk_index(x, y) for x, y in self._all_coordinates(self.target))

    def shortest_path(self):
        network = self._calculate_network()
        distances = self._calculate_distances(network)
        # self._print_shortest_route(distances)

        return distances[self.target[0], self.target[1], TORCH]

    def _calculate_network(self):
        network = defaultdict(dict)
        for x, y in self._all_coordinates(self.max_coordinates):
            terrain = self._get_risk_index(x, y)
            for tool in (NOTHING, TORCH, CLIMB):
                if tool == terrain:  # incompatible tool
                    continue

                for other_tool in (NOTHING, TORCH, CLIMB):
                    if tool != other_tool:  # Changing tool
                        network[x, y, tool][x, y, other_tool] = 7

                for n_x, n_y in self._neighbours(x, y):
                    neighbour = self._get_risk_index(n_x, n_y)
                    if neighbour != tool:  # compatible tool
                        network[x, y, tool][n_x, n_y, tool] = 1

        return network

    def _calculate_distances(self, network):
        starting_point = (0, 0, TORCH)
        exploration_points = [starting_point]
        explored = {starting_point: 0}
        next_exploration_points = {}
        stop_after_distance = None
        while exploration_points:
            for current in exploration_points:
                next_options = network[current]
                for next_point, cost in next_options.items():
                    next_distance = explored[current] + cost
                    if next_point not in explored or explored[next_point] > next_distance:
                        explored[next_point] = next_distance
                        next_exploration_points[next_point] = next_distance
                    if next_point == (self.target[0], self.target[1], TORCH):
                        stop_after_distance = next_distance

            next_distances = next_exploration_points.values()
            closest_next_distance = min(next_distances) if next_distances else None
            if stop_after_distance is not None:
                closest_next_distance = min(closest_next_distance, stop_after_distance)
            exploration_points = {
                p for p, d in next_exploration_points.items()
                if d == closest_next_distance
            }
            next_exploration_points = {
                p: d for p, d in next_exploration_points.items()
                if p not in exploration_points
            }

        return explored

    def _print_shortest_route(self, explored):
        current = self.target[0], self.target[1], TORCH
        while current != (0, 0, TORCH):
            x, y, tool = current
            next_distances = defaultdict(list)

            for other_tool in (NOTHING, TORCH, CLIMB):
                if tool != other_tool:  # Changing tool
                    next_point = x, y, other_tool
                    next_distance = explored[next_point]
                    next_distances[next_distance].append(next_point)

            for n_x, n_y in self._neighbours(x, y):
                neighbour = self._get_risk_index(n_x, n_y)
                if neighbour != tool:  # compatible tool
                    next_point = n_x, n_y, tool
                    next_distance = explored[next_point]
                    next_distances[next_distance].append(next_point)

            min_distance = min(next_distances.keys())
            current = next_distances[min_distance][0]
            print(current)

    def _get_risk_index(self, x, y):
        return self.erosions[x, y] % 3

    def _calculate_erosions(self):
        for x, y in self._all_coordinates(self.max_coordinates):
            current = x, y
            erosion = self.erosions.get(current)
            if erosion is None:
                if x == 0:
                    geological_index = y * 48271
                elif y == 0:
                    geological_index = x * 16807
                else:
                    geological_index = self.erosions[x - 1, y] * self.erosions[x, y - 1]
                erosion = (geological_index + self.depth) % 20183
                self.erosions[current] = erosion

    def _all_coordinates(self, max_coordinates):
        max_x, max_y = max_coordinates
        for x in range(max_x + 1):
            for y in range(max_y + 1):
                yield x, y

    def _neighbours(self, x, y):
        for dx in (-1, 1):
            tx = x + dx
            if 0 <= tx <= self.max_coordinates[0]:
                yield tx, y
        for dy in (-1, 1):
            ty = y + dy
            if 0 <= ty <= self.max_coordinates[1]:
                yield x, ty
