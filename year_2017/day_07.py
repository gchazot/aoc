from unittest import TestCase

from collections import defaultdict

from aoc_utils import data_file
import re


program_details_parse = re.compile("(?P<id>\w+) \((?P<weight>\d+)\)( -> (?P<children>.*))?$")


class FoundImbalance(Exception):
    def __init__(self, weight_should_be):
        self.correct_weight = weight_should_be


class ProgramTower:
    def __init__(self, data_filename):
        with open(data_file(2017, data_filename)) as shouts:
            programs_shouts = shouts.readlines()
            programs_details = map(parse_program_shout, programs_shouts)
            self.programs = dict(programs_details)

    def find_tower_base(self):
        program_ids = set(self.programs.keys())
        map(lambda p: program_ids.discard(p), gen_child_ids(self.programs))
        return list(program_ids)

    def find_correct_weight(self):
        base = self.find_tower_base()[0]
        try:
            self.calculate_total_weights(base)
        except FoundImbalance as result:
            return result.correct_weight
        return None


    def calculate_total_weights(self, root_id):
        program = self.programs[root_id]
        for child_id in program["children"]:
            program["children_weight"] += self.calculate_total_weights(child_id)

        self.check_imbalance(root_id)
        return program["weight"] + program["children_weight"]

    def check_imbalance(self, program_id):
        program = self.programs[program_id]
        weights = defaultdict(list)
        for child_id in program["children"]:
            child = self.programs[child_id]
            total_weight = child["weight"] + child["children_weight"]
            weights[total_weight].append(child_id)
        if len(weights) > 1:
            raise FoundImbalance(self.find_weight_correction(weights))

    def find_weight_correction(self, children_weights):
        correct_total_weight, _ = max(children_weights.items(), key=lambda (weight, programs): len(programs))
        wrong_total_weight, wrong_program_id = min(children_weights.items(), key=lambda (weight, programs): len(programs))
        wrong_program = self.programs[wrong_program_id[0]]
        correct_program_weight = wrong_program["weight"] + correct_total_weight - wrong_total_weight
        return correct_program_weight


def parse_program_shout(shout):
    details = program_details_parse.match(shout)
    children_group = details.group("children")
    if children_group is not None:
        children = children_group.split(", ")
    else:
        children = []
    return details.group("id"), {
        "weight": int(details.group("weight")),
        "children": children,
        "children_weight": 0}


def gen_child_ids(programs):
    for _, details in programs.iteritems():
        for child in details["children"]:
            yield child


class TestProgramTree(TestCase):
    def test_find_tower_base_example(self):
        tower = ProgramTower("day_07_example.txt")
        bases = tower.find_tower_base()
        self.assertEqual(1, len(bases))
        self.assertEqual("tknk", bases[0])

    def test_find_tower_base_mine(self):
        tower = ProgramTower("day_07_mine.txt")
        bases = tower.find_tower_base()
        self.assertEqual(1, len(bases))
        self.assertEqual("dtacyn", bases[0])

    def test_find_correct_weight_example(self):
        tower = ProgramTower("day_07_example.txt")
        self.assertEqual(60, tower.find_correct_weight())

    def test_find_correct_weight_mine(self):
        tower = ProgramTower("day_07_mine.txt")
        self.assertEqual(521, tower.find_correct_weight())
