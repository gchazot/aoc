import collections
import itertools
import re
import unittest

from aoc_utils.data import data_lines


class TestSimulate(unittest.TestCase):
    def test_simulate_example(self):
        example_lines = data_lines(2018, 'day_24_example.txt')
        groups = list(parse(example_lines))
        surviving_groups = simulate(groups)

        self.assertEqual(2, len(surviving_groups))
        self.assertTrue(all("Infection" == g.army for g in surviving_groups))
        self.assertSetEqual({782, 4434}, {g.units for g in surviving_groups})

        self.assertEqual(5216, sum(g.units for g in surviving_groups))

    def test_simulate_mine(self):
        lines = data_lines(2018, 'day_24_mine.txt')
        groups = list(parse(lines))

        surviving_groups = simulate(groups)

        self.assertTrue(all("Infection" == g.army for g in surviving_groups))

        self.assertEqual(30881, sum(g.units for g in surviving_groups))

    def test_find_smallest_boost_example(self):
        example_lines = data_lines(2018, 'day_24_example.txt')
        groups = list(parse(example_lines))
        surviving_groups = find_smallest_boost(groups)

        self.assertEqual(51, sum(g.units for g in surviving_groups))

    @unittest.skip("Too Slow")
    def test_find_smallest_boost_mine(self):
        lines = data_lines(2018, 'day_24_mine.txt')
        groups = list(parse(lines))
        surviving_groups = find_smallest_boost(groups)

        self.assertEqual(1847, sum(g.units for g in surviving_groups))


def find_smallest_boost(groups):
    boost = 0
    while True:
        survivors = simulate(groups, boost)
        if survivors and survivors[0].army == "Immune System":
            return survivors
        boost += 1


def simulate(groups, boost=0):
    for group in groups:
        group.reset(boost)

    while True:
        targets = {}
        for group in target_selection_order(groups):
            valid_targets = [
                other for other in groups
                if other.army != group.army and other not in targets.values() and other.max_damage(group) > 0
            ]
            if len(valid_targets) > 0:
                target = group.best_target(valid_targets)
                targets[group] = target

        some_kills = False
        for attacker in initiative_order(targets.keys()):
            defender = targets[attacker]
            some_kills |= defender.receive_damage(attacker)

        if not some_kills:
            return

        groups = [g for g in groups if g.units > 0]

        if len({g.army for g in groups}) <= 1:
            return groups


class TestGroup(unittest.TestCase):
    def test_effective_power(self):
        self.assertEqual(1, Group(None, 1, None, {}, 1, None, None).effective_power)
        self.assertEqual(0, Group(None, 0, None, {}, 1, None, None).effective_power)
        self.assertEqual(0, Group(None, 1, None, {}, 0, None, None).effective_power)
        self.assertEqual(10, Group(None, 1, None, {}, 10, None, None).effective_power)
        self.assertEqual(10, Group(None, 10, None, {}, 1, None, None).effective_power)

    def test_damage_factor(self):
        attack = Group(None, None, None, {}, None, 'attack_type', None)
        weak = Group(None, None, None, {'weak': ['attack_type']}, None, None, None)
        immune = Group(None, None, None, {'immune': ['attack_type']}, None, None, None)

        self.assertEqual(1, attack.damage_factor(attack))
        self.assertEqual(2, weak.damage_factor(attack))
        self.assertEqual(0, immune.damage_factor(attack))

    def test_max_damage(self):
        attack = Group(None, 4, None, {}, 25, 'attack_type', None)
        weak = Group(None, None, None, {'weak': ['attack_type']}, None, None, None)
        immune = Group(None, None, None, {'immune': ['attack_type']}, None, None, None)

        self.assertEqual(100, attack.max_damage(attack))
        self.assertEqual(200, weak.max_damage(attack))
        self.assertEqual(0, immune.max_damage(attack))

    def test_best_target_prefers_damage_first(self):
        attack = Group(None, 1, None, {}, 1, 'attack_type', None)
        weak = Group(None, 1, None, {'weak': ['attack_type']}, 1, None, None)
        immune = Group(None, 1, None, {'immune': ['attack_type']}, 1, None, None)
        groups = (attack, weak, immune)

        self.assertEqual(weak, attack.best_target(groups))

    def test_best_target_checks_effective_power_second(self):
        attack = Group(None, 1, None, {}, 0, 'attack_type', None)
        t1 = Group(None, 1, None, {}, 0, None, None)
        t2 = Group(None, 1, None, {}, 1, None, None)
        t3 = Group(None, 1, None, {}, 0, None, None)
        groups = [t1, t2, t3]

        self.assertEqual(t2, attack.best_target(groups))

    def test_best_target_uses_initiative_last(self):
        attack = Group(None, 1, None, {}, 0, 'attack_type', None)
        t1 = Group(None, 1, None, {}, 0, None, 2)
        t2 = Group(None, 1, None, {}, 0, None, 3)
        t3 = Group(None, 1, None, {}, 0, None, 1)
        groups = [t1, t2, t3]

        self.assertEqual(t2, attack.best_target(groups))

    def test_receive_damage(self):
        attacker = Group(None, 7, None, {}, 5, 'attack_type', None)
        normal = Group(None, 100, 10, {}, 0, None, None)
        weak = Group(None, 100, 10, {'weak': ['attack_type']}, 0, None, None)
        immune = Group(None, 100, 10, {'immune': ['attack_type']}, 0, None, None)

        normal.receive_damage(attacker)
        self.assertEqual(97, normal.units)

        weak.receive_damage(attacker)
        self.assertEqual(93, weak.units)

        immune.receive_damage(attacker)
        self.assertEqual(100, immune.units)

    def test_get_armies(self):
        g1 = Group("army A", None, None, {}, None, None, None)
        g2 = Group("army B", None, None, {}, None, None, None)
        g3 = Group("army A", None, None, {}, None, None, None)
        g4 = Group("army B", None, None, {}, None, None, None)
        groups = [g1, g2, g3, g4]

        armies = get_armies(groups)

        self.assertSetEqual({"army A", "army B"}, frozenset(armies.keys()))
        self.assertSetEqual({g1, g3}, armies["army A"])
        self.assertSetEqual({g2, g4}, armies["army B"])

    def test_target_selection_order(self):
        g1 = Group(None, 2, None, {}, 1, None, 1)
        g2 = Group(None, 2, None, {}, 1, None, 2)
        g3 = Group(None, 1, None, {}, 1, None, 3)
        groups = [g1, g2, g3]
        self.assertSequenceEqual((g2, g1, g3), target_selection_order(groups))

    def test_initiative_order(self):
        g1 = Group(None, None, None, {}, None, None, 1)
        g2 = Group(None, None, None, {}, None, None, 3)
        g3 = Group(None, None, None, {}, None, None, 2)
        groups = [g1, g2, g3]
        self.assertSequenceEqual((g2, g3, g1), initiative_order(groups))


class Group:
    def __init__(self, army, units, hit_points, properties, strength, attack, initiative):
        self.army = army
        self.units = units
        self.hit_points = hit_points
        self.immunities = properties.get('immune', [])
        self.weaknesses = properties.get('weak', [])
        self._strength = strength
        self.attack = attack
        self.initiative = initiative
        self.boost = 0
        self.original_units = units

    def __repr__(self):
        result = '{army}: {units} units, {hit_points} hp, {_strength} x {attack}'
        if self.weaknesses:
            result += ', weak {weaknesses}'
        if self.immunities:
            result += ', immune {immunities}'
        result += ', initiative {initiative}'
        return result.format(**self.__dict__)

    def reset(self, boost):
        self.units = self.original_units
        self.boost = boost if self.army == 'Immune System' else 0

    @property
    def strength(self):
        return self._strength + self.boost

    @property
    def effective_power(self):
        return self.units * self.strength

    @property
    def target_selection_rank(self):
        return self.effective_power, self.initiative

    def damage_factor(self, attacker):
        attack = attacker.attack
        if attack in self.immunities:
            return 0
        if attack in self.weaknesses:
            return 2
        return 1

    def max_damage(self, attacker):
        factor = self.damage_factor(attacker)
        return factor * attacker.effective_power

    def best_target(self, groups):
        return max(
            groups,
            key=lambda group: (group.damage_factor(self), group.effective_power, group.initiative)
        )

    def receive_damage(self, attacker):
        damage = self.max_damage(attacker)
        units_lost = damage // self.hit_points
        self.units = max(0, self.units - units_lost)
        return units_lost > 0


def target_selection_order(groups):
    return sorted(groups, key=lambda group: group.target_selection_rank, reverse=True)


def initiative_order(groups):
    return sorted(groups, key=lambda group: group.initiative, reverse=True)


def get_armies(groups):
    armies = collections.defaultdict(set)
    for group in groups:
        armies[group.army].add(group)
    return armies


class TestParse(unittest.TestCase):
    def test_parse_example(self):
        example_lines = data_lines(2018, 'day_24_example.txt')
        groups = list(parse(example_lines))

        self.assertEqual(4, len(groups))
        self.assertEqual(2, len({group.army for group in groups}))
        self.assertEqual(2, len({group for group in groups if group.army == 'Immune System'}))
        self.assertEqual(2, len({group for group in groups if group.army == 'Infection'}))

        self.assertSetEqual({17, 989, 801, 4485}, {group.units for group in groups})
        self.assertSetEqual({5390, 1274, 4706, 2961}, {group.hit_points for group in groups})
        self.assertSetEqual(
            {'radiation', 'fire', 'bludgeoning', 'slashing', 'cold'},
            frozenset(itertools.chain.from_iterable(group.weaknesses for group in groups)),
        )
        self.assertSetEqual(
            {'radiation', 'fire'},
            frozenset(itertools.chain.from_iterable(group.immunities for group in groups)),
        )

    def test_mine(self):
        self.maxDiff = None
        example_lines = data_lines(2018, 'day_24_mine.txt')
        groups = list(parse(example_lines))
        expected = {
            "Immune System: 1514 units, 8968 hp, 57 x bludgeoning, weak ['cold'], initiative 9",
            "Immune System: 2721 units, 6691 hp, 22 x slashing, weak ['cold'], initiative 15",
            "Immune System: 1214 units, 10379 hp, 69 x fire, immune ['bludgeoning'], initiative 16",
            "Immune System: 2870 units, 4212 hp, 11 x radiation, initiative 12",
            "Immune System: 1239 units, 5405 hp, 37 x cold, weak ['cold'], initiative 18",
            "Immune System: 4509 units, 4004 hp, 8 x slashing, weak ['cold'], immune ['radiation'], initiative 20",
            "Immune System: 3369 units, 10672 hp, 29 x cold, weak ['slashing'], initiative 11",
            "Immune System: 2890 units, 11418 hp, 30 x cold, weak ['fire'], immune ['bludgeoning'], initiative 8",
            "Immune System: 149 units, 7022 hp, 393 x radiation, weak ['slashing'], initiative 13",
            "Immune System: 2080 units, 5786 hp, 20 x fire, weak ['fire'], immune ['slashing', 'bludgeoning'], initiative 7",
            "Infection: 817 units, 47082 hp, 115 x cold, immune ['slashing', 'radiation', 'bludgeoning'], initiative 3",
            "Infection: 4183 units, 35892 hp, 16 x bludgeoning, initiative 1",
            "Infection: 7006 units, 11084 hp, 2 x fire, initiative 2",
            "Infection: 4804 units, 25411 hp, 10 x cold, initiative 14",
            "Infection: 6262 units, 28952 hp, 7 x slashing, weak ['fire'], initiative 10",
            "Infection: 628 units, 32906 hp, 99 x radiation, weak ['slashing'], initiative 4",
            "Infection: 5239 units, 46047 hp, 14 x bludgeoning, immune ['fire'], initiative 6",
            "Infection: 1173 units, 32300 hp, 53 x bludgeoning, weak ['cold', 'slashing'], initiative 19",
            "Infection: 3712 units, 12148 hp, 5 x slashing, weak ['slashing'], immune ['cold'], initiative 17",
            "Infection: 334 units, 43582 hp, 260 x cold, weak ['cold', 'fire'], initiative 5",
        }
        self.assertSetEqual(expected, {repr(g) for g in groups})


group_pattern = re.compile(r'^(?P<units>.+?)( \((?P<properties>[^)]+)\))? with an(?P<strength>.*)$')
unit_pattern = re.compile(r'(?P<units>\d+)\D+(?P<points>\d+)\D+')
strength_pattern = re.compile(r'\D+(?P<attack>\d+) (?P<type>\w+) damage.*?(?P<initiative>\d+)$')
property_pattern = re.compile(r'(?P<style>\w+) to (?P<types>.*)')


def parse(lines):
    army = None
    for line in lines:
        if len(line) == 0:
            continue
        elif line.endswith(':'):
            army = line[:-1]
            continue

        group_match = group_pattern.match(line)
        units_match = unit_pattern.match(group_match.group('units'))
        units = int(units_match.group('units'))
        points = int(units_match.group('points'))

        strength_match = strength_pattern.match(group_match.group('strength'))
        strength = int(strength_match.group('attack'))
        attack = strength_match.group('type')
        initiative = int(strength_match.group('initiative'))

        properties_match = group_match.group('properties')
        if properties_match:
            property_styles = properties_match.split('; ')
            property_matches = [property_pattern.match(description) for description in property_styles]
            properties = {
                match.group('style'): match.group('types').split(', ')
                for match in property_matches
            }
        else:
            properties = {}

        yield Group(army, units, points, properties, strength, attack, initiative)
