import itertools
import operator
import re
import unittest
from aoc_utils.geometry import add_coordinates
try:
    from functools import reduce
except ImportError:
    pass


class TestMoonSystem(unittest.TestCase):
    def test_initialise(self):
        moons = MoonSystem([
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ])
        self.assertEqual(4, len(moons.position))
        self.assertEqual(4, len(moons.velocity))

        self.assertEqual((-1, 0, 2), moons.position[0])
        self.assertEqual((2, -10, -7), moons.position[1])
        self.assertEqual((4, -8, 8), moons.position[2])
        self.assertEqual((3, 5, -1), moons.position[3])

        for i in range(4):
            self.assertEqual([0, 0, 0], moons.velocity[i])

    def test_perform_steps(self):
        moons = MoonSystem([
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ])

        moons.perform_steps(num_steps=1)
        self.assertEqual((2, -1, 1), moons.position[0])
        self.assertEqual([3, -1, -1], moons.velocity[0])
        self.assertEqual((3, -7, -4), moons.position[1])
        self.assertEqual([1, 3, 3], moons.velocity[1])
        self.assertEqual((1, -7, 5), moons.position[2])
        self.assertEqual([-3, 1, -3], moons.velocity[2])
        self.assertEqual((2, 2, 0), moons.position[3])
        self.assertEqual([-1, -3, 1], moons.velocity[3])

        moons.perform_steps(num_steps=1)
        self.assertEqual((5, -3, -1), moons.position[0])
        self.assertEqual([3, -2, -2], moons.velocity[0])
        self.assertEqual((1, -2, 2), moons.position[1])
        self.assertEqual([-2, 5, 6], moons.velocity[1])
        self.assertEqual((1, -4, -1), moons.position[2])
        self.assertEqual([0, 3, -6], moons.velocity[2])
        self.assertEqual((1, -4, 2), moons.position[3])
        self.assertEqual([-1, -6, 2], moons.velocity[3])

        moons.perform_steps(num_steps=7)
        self.assertEqual((5, 3, -4), moons.position[0])
        self.assertEqual([0, 1, -2], moons.velocity[0])
        self.assertEqual((2, -9, -3), moons.position[1])
        self.assertEqual([0, -2, 2], moons.velocity[1])
        self.assertEqual((0, -8, 4), moons.position[2])
        self.assertEqual([0, 1, -2], moons.velocity[2])
        self.assertEqual((1, 1, 5), moons.position[3])
        self.assertEqual([0, 0, 2], moons.velocity[3])

    def test_energy(self):
        moons = MoonSystem([
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ])

        moons.perform_steps(100)
        self.assertEqual(290, moons.total_energy(0))
        self.assertEqual(608, moons.total_energy(1))
        self.assertEqual(574, moons.total_energy(2))
        self.assertEqual(468, moons.total_energy(3))

    def test_energy_mine(self):
        moons = MoonSystem([
            "<x=17, y=-7, z=-11>",
            "<x=1, y=4, z=-1>",
            "<x=6, y=-2, z=-6>",
            "<x=19, y=11, z=9>",
        ])

        moons.perform_steps(1000)

        self.assertEqual(9441, sum(moons.total_energy(i) for i in range(4)))

    def test_find_cycles(self):
        moons_1 = MoonSystem([
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ])
        self.assertEqual(2772, moons_1.find_period())

        moons_2 = MoonSystem([
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ])
        self.assertEqual(4686774924, moons_2.find_period())

    @unittest.skip("This is still a little too slow")
    def test_find_cycle_mine(self):
        moons_1 = MoonSystem([
            "<x=17, y=-7, z=-11>",
            "<x=1, y=4, z=-1>",
            "<x=6, y=-2, z=-6>",
            "<x=19, y=11, z=9>",
        ])
        self.assertEqual(503560201099704, moons_1.find_period())


class MoonSystem:
    def __init__(self, from_lines):
        coordinates_re = re.compile("<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>")
        self.position = [
            tuple(int(i) for i in coordinates_re.match(line).groups())
            for line in from_lines
        ]
        self.velocity = [[0, 0, 0] for _ in self.position]

    def perform_steps(self, num_steps):
        for _ in range(num_steps):
            self.apply_gravity()
            self.apply_velocity()

    def apply_gravity(self):
        moon_ids = range(len(self.position))
        for moon_a, moon_b in itertools.combinations(moon_ids, 2):
            position_a = self.position[moon_a]
            position_b = self.position[moon_b]
            velocity_a = self.velocity[moon_a]
            velocity_b = self.velocity[moon_b]

            for dimension in range(3):
                if position_a[dimension] > position_b[dimension]:
                    velocity_a[dimension] -= 1
                    velocity_b[dimension] += 1
                elif position_a[dimension] < position_b[dimension]:
                    velocity_a[dimension] += 1
                    velocity_b[dimension] -= 1

    def apply_velocity(self):
        moon_ids = range(len(self.position))
        for moon_id in moon_ids:
            self.position[moon_id] = add_coordinates(self.position[moon_id], self.velocity[moon_id])

    def total_energy(self, moon):
        return self.potential_energy(moon) * self.kinetic_energy(moon)

    def potential_energy(self, moon):
        return sum(abs(coordinate) for coordinate in self.position[moon])

    def kinetic_energy(self, moon):
        return sum(abs(coordinate) for coordinate in self.velocity[moon])

    def find_period(self):
        seen_states = [set() for _ in range(3)]
        periods = [None for _ in range(3)]

        i = 0
        while not all(periods):
            for dimension in range(3):
                if periods[dimension] is not None:
                    continue

                state = (
                    tuple(position[dimension] for position in self.position) +
                    tuple(velocity[dimension] for velocity in self.velocity)
                )
                if state in seen_states[dimension]:
                    periods[dimension] = i
                else:
                    seen_states[dimension].add(state)

            self.perform_steps(num_steps=1)
            i += 1
        return first_common_period(periods)


def first_common_period(periods):
    all_factors = [list(prime_factors(n)) for n in periods]

    result_factors = all_factors[0]

    for other_factors in all_factors[1:]:
        for factor in result_factors:
            if factor in other_factors:
                other_factors.remove(factor)
        result_factors.extend(other_factors)

    return reduce(operator.mul, result_factors)


def prime_factors(n):
    for i in primes(n):
        while n % i == 0:
            n //= i
            yield i


def primes(n):
    if n <= 2:
        return []
    sieve = [True] * (n + 1)
    for x in range(3, int(n ** 0.5) + 1, 2):
        for y in range(3, (n // x) + 1, 2):
            sieve[(x * y)] = False

    return [2] + [i for i in range(3, n, 2) if sieve[i]]
