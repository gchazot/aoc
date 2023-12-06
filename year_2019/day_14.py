import unittest

from aoc_utils.data import data_lines


class TestReactions(unittest.TestCase):
    example_1 = [
        "10 ORE => 10 A",
        "1 ORE => 1 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "7 A, 1 E => 1 FUEL",
    ]

    def test_parse(self):
        reactions = parse(self.example_1)

        self.assertEqual(6, len(reactions))
        self.assertEqual(
            ((10, "A"), {(10, 'ORE')}),
            reactions[0],
        )
        self.assertEqual(
            ((1, "FUEL"), {(7, 'A'), (1, 'E')}),
            reactions[5],
        )

    def test_reaction_book(self):
        reactions = parse(self.example_1)
        book = ReactionBook(reactions)

        self.assertEqual(10, book.ore_needed_for(1, 'A'))
        self.assertEqual(10, book.ore_needed_for(10, 'A'))
        self.assertEqual(1, book.ore_needed_for(1, 'B'))
        self.assertEqual(11, book.ore_needed_for(1, 'C'))

        self.assertEqual(31, book.ore_needed_for(1, 'FUEL'))

        book2 = ReactionBook(parse([
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ]))
        self.assertEqual(165, book2.ore_needed_for(1, "FUEL"))

        book3 = ReactionBook(parse([
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ]))
        self.assertEqual(13312, book3.ore_needed_for(1, "FUEL"))

        book4 = ReactionBook(parse([
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ]))
        self.assertEqual(180697, book4.ore_needed_for(1, "FUEL"))

        book5 = ReactionBook(parse([
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ]))
        self.assertEqual(2210736, book5.ore_needed_for(1, "FUEL"))

    def test_reaction_book_mine(self):
        book2 = ReactionBook(parse(data_lines(2019, "day_14_mine.txt")))
        self.assertEqual(741927, book2.ore_needed_for(1, "FUEL"))

        available_ore = 1000000000000
        upper = 2

        while book2.ore_needed_for(upper, "FUEL") <= available_ore:
            upper *= 2
        lower = upper // 2

        while upper - lower > 1:
            median = (upper + lower) // 2
            ore_needed = book2.ore_needed_for(median, "FUEL")
            if ore_needed < available_ore:
                lower = median
            else:
                upper = median

        self.assertEqual(999999550766, book2.ore_needed_for(lower, "FUEL"))
        self.assertEqual(1000000157785, book2.ore_needed_for(upper, "FUEL"))
        self.assertEqual(2371699, lower)


def parse(reaction_lines):
    return [parse_reaction(line) for line in reaction_lines]


def parse_reaction(reaction_line):
    all_ingredients, outcome = reaction_line.split(" => ")
    ingredients = all_ingredients.split(", ")

    outcome_split = split_ingredient(outcome)
    ingredients_split = {
        split_ingredient(ingredient)
        for ingredient in ingredients
    }

    return outcome_split, ingredients_split


def split_ingredient(ingredient):
    count_str, item = ingredient.split(" ")
    return int(count_str), item


class ReactionBook:
    def __init__(self, reactions):
        self._reactions = reactions
        self._produce = {
            reaction[0][1]: (
                reaction[0][0],
                {
                    ingredient: ingredient_quantity
                    for ingredient_quantity, ingredient in reaction[1]
                }
            )
            for reaction in self._reactions
        }

    def ore_needed_for(self, quantity, material):
        needed = {material: quantity}
        while any(
            quantity_needed > 0
            for material_needed, quantity_needed in needed.items()
            if material_needed != "ORE"
        ):
            for needed_material, needed_quantity in list(needed.items()):
                if needed_material == "ORE" or needed_quantity <= 0:
                    continue

                produced_quantity, requirements = self._produce[needed_material]

                number_of_times = needed[needed_material] // produced_quantity
                if needed[needed_material] % produced_quantity > 0:
                    number_of_times += 1
                needed[needed_material] -= number_of_times * produced_quantity

                for ingredient, ingredient_quantity in requirements.items():
                    needed.setdefault(ingredient, 0)
                    needed[ingredient] += ingredient_quantity * number_of_times

        return needed["ORE"]
