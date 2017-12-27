import unittest
import re
from aoc_utils import data_file


coordinates_pattern = re.compile("[pva]=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>")


class Particle:
    def __init__(self, name, text):
        coordinates = coordinates_pattern.findall(text)
        self.name = name
        self.position = list(map(int, coordinates[0]))
        self.speed = list(map(int, coordinates[1]))
        self.acceleration = list(map(int, coordinates[2]))

    def manhattan_distance(self):
        return sum(map(abs, self.position))

    def manhattan_speed(self):
        return sum(map(abs, self.speed))

    def manhattan_acceleration(self):
        return sum(map(abs, self.acceleration))

    def closer_than(self, other):
        if self.manhattan_acceleration() < other.manhattan_acceleration():
            return True
        elif self.manhattan_acceleration() > other.manhattan_acceleration():
            return False
        elif self.manhattan_speed() < other.manhattan_speed():
            return True
        elif self.manhattan_speed() > other.manhattan_speed():
            return False
        elif self.manhattan_distance() < other.manhattan_distance():
            return True

    @staticmethod
    def gen_from_file(filename):
        with open(data_file(2017, filename)) as f:
            for i, line in enumerate(f.readlines()):
                yield Particle(i, line)

    @staticmethod
    def find_closest(particles):
        closest = None
        for particle in particles:
            if closest is None or particle.closer_than(closest):
                closest = particle

        return closest


class TestParticle(unittest.TestCase):
    def test_init_Particle(self):
        part1 = Particle(1, "p=< 1,2,3>, v=< 4,5,6>, a=< -7,-8,-9>")
        self.assertEqual(1, part1.name)
        self.assertListEqual([1, 2, 3], part1.position)
        self.assertListEqual([4, 5, 6], part1.speed)
        self.assertListEqual([-7, -8, -9], part1.acceleration)

        part2 = Particle(2, "p=< -1,-2,-3>, v=< -4,-5,-6>, a=< 7,8,9>")
        self.assertEqual(2, part2.name)
        self.assertListEqual([-1, -2, -3], part2.position)
        self.assertListEqual([-4, -5, -6], part2.speed)
        self.assertListEqual([7, 8, 9], part2.acceleration)

    def test_find_closest_example(self):
        p0 = Particle(0, "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>")
        p1 = Particle(1, "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>")
        self.assertEqual(p0, Particle.find_closest([p0, p1]))
        self.assertEqual(p0, Particle.find_closest([p1, p0]))

    def test_read_from_file(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        self.assertEqual(1000, len(list(particles)))

    def test_find_closest_mine(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        closest = Particle.find_closest(particles)

        self.assertEqual(243, closest.name)
