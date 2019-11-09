import collections
import itertools
import math
import unittest

import aoc_utils.geometry
from aoc_utils import char_map, data


class TestCoordinatesUtils(unittest.TestCase):
    def test_solve_tie(self):
        self.assertEqual(None, solve_tie([]))
        self.assertEqual((12, 34), solve_tie([(12, 34)]))
        self.assertEqual((1, 1), solve_tie([(1, 1), (2, 2)]))
        self.assertEqual((1, 1), solve_tie([(2, 2), (1, 1)]))
        self.assertEqual((2, 1), solve_tie([(1, 2), (2, 1)]))
        self.assertEqual((2, 1), solve_tie([(2, 1), (1, 2)]))


def solve_tie(options):
    if len(options):
        return sorted_by_priority(options)[0]


def sorted_by_priority(options):
    return sorted(options, key=reverse_coordinates)


def reverse_coordinates(coordinates):
    return tuple(i for i in reversed(coordinates))


class FindAllClosestRules(char_map.ProgressRules):
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

    def solve_tie(self, coordinate_options):
        return solve_tie(coordinate_options)


class TestCaves(unittest.TestCase):
    def make_default_caves(self):
        caves = Caves([
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        return caves

    def test_init_fighters(self):
        caves = self.make_default_caves()
        fighters = caves.fighters

        self.assertSetEqual({'E', 'G'}, set(fighters.keys()))
        self.assertEqual({(1, 1): 200}, fighters['E'])
        self.assertEqual({(4, 1): 200, (2, 3): 200, (5, 3): 200}, fighters['G'])

    def test_get_targets(self):
        caves = self.make_default_caves()

        self.assertListEqual([(4, 1), (2, 3), (5, 3)], list(caves.get_targets("E")))
        self.assertListEqual([(1, 1)], list(caves.get_targets("G")))

    def test_get_in_range(self):
        caves = self.make_default_caves()

        self.assertListEqual([(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
                             list(caves.get_in_range("E")))
        self.assertListEqual([(2, 1), (1, 2)],
                             list(caves.get_in_range("G")))

    def test_get_coordinates_around(self):
        caves = self.make_default_caves()
        self.assertListEqual([(2, 1), (1, 2)], list(caves.get_coordinates_around((1, 1))))
        self.assertListEqual([(3, 1), (5, 1)], list(caves.get_coordinates_around((4, 1))))
        self.assertListEqual([(2, 2), (1, 3), (3, 3)], list(caves.get_coordinates_around((2, 3))))
        self.assertListEqual([(5, 2)], list(caves.get_coordinates_around((5, 3))))

    def test_find_all_closest_rules(self):
        caves = Caves([
            "#######",
            "#E#.G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        finder = char_map.MapExplorer(caves._caves)
        rules = FindAllClosestRules(
            targets=[(3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3)],
            allowed_values=[EMPTY_VALUE]
        )
        finder.explore(start_point=(1, 1), rules=rules)
        self.assertListEqual([(2, 2), (1, 3)], list(rules.results))

    def test_iterate_units(self):
        caves = self.make_default_caves()
        self.assertListEqual([(1, 1), (4, 1), (2, 3), (5, 3)], caves.iterate_units())

    def test_get_attack_target(self):
        caves_2 = Caves([
            "#######",
            "#..EG.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ])
        self.assertEqual((4, 1), caves_2.get_attack_target((3, 1), 'E'))
        self.assertEqual((3, 1), caves_2.get_attack_target((4, 1), 'G'))
        self.assertEqual(None, caves_2.get_attack_target((2, 3), 'G'))
        self.assertEqual(None, caves_2.get_attack_target((5, 3), 'G'))

    def test_find_next_step(self):
        caves = self.make_default_caves()
        self.assertEqual((2, 1), caves.find_next_step((1, 1), 'E'))
        self.assertEqual((3, 1), caves.find_next_step((4, 1), 'G'))
        self.assertEqual((2, 2), caves.find_next_step((2, 3), 'G'))
        self.assertEqual(None, caves.find_next_step((5, 3), 'G'))

    def test_play_unit(self):
        caves = self.make_default_caves()
        fighters = caves.fighters

        caves.play_unit((1, 1), 'E')
        self.assertEqual({(2, 1): 200}, fighters['E'])
        self.assertEqual({(4, 1): 200, (2, 3): 200, (5, 3): 200}, fighters['G'])

        caves.play_unit((2, 1), 'E')
        self.assertEqual({(3, 1): 200}, fighters['E'])
        self.assertEqual({(4, 1): 197, (2, 3): 200, (5, 3): 200}, fighters['G'])

        for _ in range(65):
            caves.play_unit((3, 1), 'E')
        self.assertEqual({(3, 1): 200}, fighters['E'])
        self.assertEqual({(4, 1): 2, (2, 3): 200, (5, 3): 200}, fighters['G'])

        caves.play_unit((3, 1), 'E')
        self.assertEqual({(3, 1): 200}, fighters['E'])
        self.assertEqual({(2, 3): 200, (5, 3): 200}, fighters['G'])

    def test_play_round(self):
        caves = self.make_default_caves()
        fighters = caves.fighters

        self.assertFalse(caves.play_round())
        self.assertEqual({(2, 1): 194}, fighters['E'])
        self.assertEqual({(3, 1): 200, (2, 2): 200, (5, 3): 200}, fighters['G'])

        self.assertTrue(caves.play_round())
        self.assertEqual({(2, 1): 188}, fighters['E'])
        self.assertEqual({(3, 1): 197, (2, 2): 200, (5, 3): 200}, fighters['G'])

        for _ in range(31):
            self.assertTrue(caves.play_round())
        self.assertEqual({(2, 1): 2}, fighters['E'])
        self.assertEqual({(3, 1): 104, (2, 2): 200, (5, 3): 200}, fighters['G'])

        self.assertRaises(FightIsOver, caves.play_round)
        self.assertEqual({}, fighters['E'])
        self.assertEqual({(3, 1): 101, (2, 2): 200, (5, 3): 200}, fighters['G'])

    def test_play(self):
        caves = self.make_default_caves()
        fighters = caves.fighters

        self.assertEqual(16533, caves.play())
        self.assertEqual({}, fighters['E'])
        self.assertEqual({(3, 1): 101, (2, 2): 200, (5, 3): 200}, fighters['G'])

    def test_play_examples(self):
        def check(expected_outcome, cave_lines, echo=False):
            caves = Caves(cave_lines)
            outcome = caves.play()
            if echo:
                caves.echo()
            self.assertEqual(expected_outcome, outcome)

        check(27730, [
            '#######',
            '#.G...#',
            '#...EG#',
            '#.#.#G#',
            '#..G#E#',
            '#.....#',
            '#######',
        ])

        check(36334, [
            '#######',
            '#G..#E#',
            '#E#E.E#',
            '#G.##.#',
            '#...#E#',
            '#...E.#',
            '#######',
        ])

        check(39514, [
            '#######',
            '#E..EG#',
            '#.#G.E#',
            '#E.##E#',
            '#G..#.#',
            '#..E#.#',
            '#######',
        ])

        check(27755, [
            '#######',
            '#E.G#.#',
            '#.#G..#',
            '#G.#.G#',
            '#G..#.#',
            '#...E.#',
            '#######',
        ])

        check(28944, [
            '#######',
            '#.E...#',
            '#.#..G#',
            '#.###.#',
            '#E#G#G#',
            '#...#G#',
            '#######',
        ])

        check(18740, [
            '#########',
            '#G......#',
            '#.E.#...#',
            '#..##..G#',
            '#...##..#',
            '#...#...#',
            '#.G...G.#',
            '#.....G.#',
            '#########',
        ])

    def test_play_mine(self):
        caves_lines = data.data_lines(2018, "day_15_mine.txt")
        caves = Caves(caves_lines)

        outcome = caves.play()
        self.assertEqual(201123, outcome)

    def test_find_minimum_elves_strength(self):
        for elf_strength in range(13, 20):
            strengths = {'E': elf_strength, 'G': 3}
            caves_lines = data.data_lines(2018, "day_15_mine.txt")
            caves = Caves(caves_lines, teams_strength=strengths)

            num_elves = len(caves.fighters['E'])

            outcome = caves.play()
            if len(caves.fighters['E']) == num_elves:
                break

        self.assertEqual(14, elf_strength)
        self.assertEqual(54188, outcome)


TEAMS_STRENGTH = {'E': 3, 'G': 3}
EMPTY_VALUE = '.'
WALL_VALUE = '#'


class FightIsOver(Exception):
    pass


class Caves:
    def __init__(self, initial_map, teams_strength=TEAMS_STRENGTH):
        self._caves = char_map.CharMap(input_lines=initial_map)
        self.strength = teams_strength
        self.fighters = {team: {} for team in teams_strength}
        for position, entry in self._caves.items():
            if entry in teams_strength:
                self.fighters[entry][position] = 200

    def play(self):
        rounds = 0
        while True:
            try:
                nobody_moved = self.play_round()
                rounds += 1
            except FightIsOver:
                break
            if nobody_moved:
                rounds += self.play_frozen_situation()
        remaining_hit_points = sum(hp for team in self.fighters.values() for hp in team.values())
        return rounds * remaining_hit_points

    def play_round(self):
        nobody_moved = True
        for unit in self.iterate_units():
            if not self.game_on():
                raise FightIsOver
            team = self._caves[unit]
            if team == EMPTY_VALUE:
                continue
            nobody_moved = self.play_unit(unit, team) and nobody_moved

        return nobody_moved

    def play_frozen_situation(self):
        attackers = collections.defaultdict(lambda: 0)
        for unit in self.iterate_units():
            team = self._caves[unit]
            target = self.get_attack_target(unit, team)
            attackers[target] += self.strength[team]

        rounds = min(
            math.floor(self.fighters[self._caves[unit]][unit] / attackers[unit])
            for unit in self.iterate_units()
            if attackers[unit] > 0
        )

        for unit in self.iterate_units():
            team = self._caves[unit]
            self.fighters[team][unit] -= rounds * attackers[unit]

        return rounds

    def game_on(self):
        return all(team for team in self.fighters.values())

    def play_unit(self, unit, team):
        attack_target = self.get_attack_target(unit, team)
        if attack_target:
            return self.attack(attack_target, self.strength[team])

        new_position = self.find_next_step(unit, team)
        if new_position:
            self.move_unit(team, unit, new_position)

            attack_target = self.get_attack_target(new_position, team)
            if attack_target:
                return self.attack(attack_target, self.strength[team])
            return False
        return True

    def attack(self, unit, strength):
        target_team = self._caves[unit]
        self.fighters[target_team][unit] -= strength
        if self.fighters[target_team][unit] <= 0:
            del self.fighters[target_team][unit]
            self._caves[unit] = EMPTY_VALUE
            return False
        return True

    def move_unit(self, team, from_coordinates, to_coordinates):
        self._caves[to_coordinates] = team
        self._caves[from_coordinates] = EMPTY_VALUE
        self.fighters[team][to_coordinates] = self.fighters[team][from_coordinates]
        del self.fighters[team][from_coordinates]

    def get_attack_target(self, unit, team):
        adjacents = []
        min_hp = None
        for adjacent in self.get_coordinates_around(unit):
            opponent = self._caves[adjacent]
            if opponent in [EMPTY_VALUE, team]:
                continue
            hp = self.fighters[opponent][adjacent]
            if min_hp is None or hp < min_hp:
                min_hp = hp
                adjacents = [adjacent]
            elif hp == min_hp:
                adjacents.append(adjacent)
        return solve_tie(adjacents)

    def find_next_step(self, unit, team):
        in_range = self.get_in_range(team)
        if not in_range:
            return None

        finder = char_map.MapExplorer(self._caves)
        rules = FindAllClosestRules(targets=in_range, allowed_values=[EMPTY_VALUE])
        finder.explore(unit, rules)

        closest = solve_tie(rules.results)
        if not closest:
            return None

        path = finder.shortest_path(start_point=unit, end_point=closest, rules=rules)
        return path[1]

    def iterate_units(self):
        all_units = itertools.chain.from_iterable(team.keys() for team in self.fighters.values())
        return sorted_by_priority(all_units)

    def get_coordinates_around(self, coordinates):
        for delta in char_map.ADJACENT_COORDINATES_DELTAS:
            adjacent = aoc_utils.geometry.add_coordinates(coordinates, delta)
            if adjacent in self._caves and self._caves[adjacent] != WALL_VALUE:
                yield adjacent

    def get_in_range(self, opponent):
        in_range = []
        for target in self.get_targets(opponent):
            for coordinates in self.get_coordinates_around(target):
                if self._caves[coordinates] == EMPTY_VALUE:
                    in_range.append(coordinates)
        return sorted(in_range, key=lambda tup: (tup[1], tup[0]))

    def get_targets(self, opponent):
        for coordinates, entry in self._caves.items():
            if entry not in [WALL_VALUE, EMPTY_VALUE, opponent]:
                yield coordinates

    def echo(self):
        all_fighters = {unit: hp for team in self.fighters.values() for unit, hp in team.items()}
        for y, line in enumerate(self._caves.lines()):
            line += " "
            line_units = sorted_by_priority(unit for unit in all_fighters if unit[1] == y)
            line += " ".join(str(all_fighters[unit]) for unit in line_units)
            print(line)
