import collections
import math
import operator
import unittest
import re

from aoc_utils.data import data_lines

coordinates_pattern = re.compile("[pva]=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>")


def scale_vector(factor, vector):
    return list(map(lambda x: factor * x, vector))


def add_vectors(u, v):
    return list(map(operator.add, u, v))


class Particle:
    def __init__(self, name, text):
        coordinates = coordinates_pattern.findall(text)
        self.name = name
        self.position = list(map(int, coordinates[0]))
        self.velocity = list(map(int, coordinates[1]))
        self.acceleration = list(map(int, coordinates[2]))

    def manhattan_distance(self):
        return sum(map(abs, self.position))

    def manhattan_speed(self):
        return sum(map(abs, self.velocity))

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

    def position_at(self, time):
        v = scale_vector(time, self.velocity)
        w = scale_vector(time * (time + 1) / 2, self.acceleration)

        return add_vectors(self.position, add_vectors(v, w))

    def collides_with_at(self, other, time):
        return self.position_at(time) == other.position_at(time)

    def collision_times_with(self, other):
        solutions = []
        for dimension in (0, 1, 2):
            same_position_times = self._same_position_times_for_dimension(other, dimension)
            if same_position_times is not None:
                if len(same_position_times) > 0:
                    solutions.append(same_position_times)
                else:
                    return []

        options = set(solutions[0])
        for other_options in solutions[1:]:
            options.intersection_update(other_options)

        return list(options)

    def _same_position_times_for_dimension(self, other, dimension):
        delta_x = self.position[dimension] - other.position[dimension]
        delta_v = self.velocity[dimension] - other.velocity[dimension]
        delta_a = self.acceleration[dimension] - other.acceleration[dimension]

        if delta_a == 0:
            if delta_v == 0:
                if delta_x == 0:
                    # Same point: always collide
                    return None
                else:
                    # Different immobile points: never collide
                    return []
            else:
                solution = -delta_x / delta_v
                return [solution]
        else:
            discriminant = (delta_v + delta_a / 2.0) ** 2.0 - 2.0 * delta_x * delta_a
            if discriminant < 0:
                # There is no root: never collide
                return []

            discriminant_sqrt = math.sqrt(discriminant)
            solution_1 = (-delta_v - delta_a / 2.0 - discriminant_sqrt) / delta_a
            solution_2 = (-delta_v - delta_a / 2.0 + discriminant_sqrt) / delta_a

            return [solution_1, solution_2]

    @staticmethod
    def gen_from_file(filename):
        for i, line in enumerate(data_lines(2017, filename)):
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
        self.assertListEqual([4, 5, 6], part1.velocity)
        self.assertListEqual([-7, -8, -9], part1.acceleration)

        part2 = Particle(2, "p=< -1,-2,-3>, v=< -4,-5,-6>, a=< 7,8,9>")
        self.assertEqual(2, part2.name)
        self.assertListEqual([-1, -2, -3], part2.position)
        self.assertListEqual([-4, -5, -6], part2.velocity)
        self.assertListEqual([7, 8, 9], part2.acceleration)

    def test_find_closest_example(self):
        p0 = Particle(0, "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>")
        p1 = Particle(1, "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>")
        self.assertEqual(p0, Particle.find_closest([p0, p1]))
        self.assertEqual(p0, Particle.find_closest([p1, p0]))

    def test_read_from_file(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        self.assertEqual(1000, len(list(particles)))


class TestParticleAlgebraic(unittest.TestCase):
    def test_find_closest_mine(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        closest = Particle.find_closest(particles)

        self.assertEqual(243, closest.name)


class TestParticlesCollide(unittest.TestCase):
    def test_position_at_given_time(self):
        part1 = Particle(1, "p=< 1,2,3>, v=< 4,5,6>, a=< -7,-8,-9>")
        self.assertListEqual([1, 2, 3], part1.position)
        self.assertListEqual([1, 2, 3], part1.position_at(0))
        self.assertListEqual([-2, -1, 0], part1.position_at(1))
        self.assertListEqual([-12, -12, -12], part1.position_at(2))

        part2 = Particle(2, "p=< -1,-2,-3>, v=< -4,-5,-6>, a=< 7,8,9>")
        self.assertListEqual([-1, -2, -3], part2.position)
        self.assertListEqual([-1, -2, -3], part2.position_at(0))
        self.assertListEqual([2, 1, 0], part2.position_at(1))
        self.assertListEqual([12, 12, 12], part2.position_at(2))

    def test_collision_at_given_time(self):
        parts = [
            Particle(1, "p=< -6,0,0>, v=< 3,0,0>, a=< 0,0,0>"),
            Particle(2, "p=< -4,0,0>, v=< 2,0,0>, a=< 0,0,0>"),
            Particle(3, "p=< -2,0,0>, v=< 1,0,0>, a=< 0,0,0>"),
            Particle(4, "p=< 3,0,0> , v=< -1,0,0>, a=< 0,0,0>"),
        ]
        num_ticks = 3
        expected_collisions = [
            (0, 1, 2),
            (0, 2, 2),
            (1, 0, 2),
            (1, 2, 2),
            (2, 0, 2),
            (2, 1, 2),
        ]
        num_particles = len(parts)

        def assert_collision(expected, index_a, index_b, at_time):
            part_a = parts[index_a]
            part_b = parts[index_b]
            collides = part_a.collides_with_at(part_b, at_time)
            self.assertEqual(expected, collides)

        for a in range(num_particles):
            for b in range(num_particles):
                if a == b:
                    continue
                for time in range(num_ticks):
                    expect_collision = (a, b, time) in expected_collisions
                    assert_collision(expect_collision, a, b, time)


class ParticleColliderStepByStep:
    def __init__(self, particles):
        self.particles = list(particles)
        self.time = 0

    def execute(self, timeout):
        last_count = len(self.particles)

        while timeout > 0:
            self.step()

            new_count = len(self.particles)
            if new_count <= 1:
                return
            elif new_count != last_count:
                last_count = new_count
                timeout = 10
            else:
                timeout -= 1

    def step(self):
        self.time += 1
        self.clear_collisions()

    def clear_collisions(self):
        collisions = set()
        for i, particle_1 in enumerate(self.particles):
            for j, particle_2 in enumerate(self.particles):
                if i >= j:
                    continue
                if particle_1.collides_with_at(particle_2, self.time):
                    collisions.add(i)
                    collisions.add(j)
        for index in reversed(sorted(collisions)):
            self.particles.pop(index)


class TestParticleColliderStepByStep(unittest.TestCase):
    def setUp(self):
        self.particles = [
            Particle(0, "p=< 3,0,0>, v=< 1,0,0>, a=<0,0,0>"),
            Particle(1, "p=< 4,0,0>, v=< 0,0,0>, a=<0,0,0>"),
            Particle(2, "p=< -5,0,0>, v=< -1,0,0>, a=<0,0,0>"),
            Particle(3, "p=< 5,0,0>, v=< -1,0,0>, a=<0,0,0>"),
        ]
        self.collider = ParticleColliderStepByStep(self.particles)

    def test_init(self):
        self.assertEqual(4, len(self.collider.particles))

    def test_destroys_particles_on_collision(self):
        self.assertEqual(4, len(self.collider.particles))

        self.collider.step()
        self.assertEqual(1, len(self.collider.particles))

    def test_destroys_particles_until_timeout(self):
        self.collider.execute(timeout=10)
        self.assertEqual(1, len(self.collider.particles))
        self.assertEqual(1, self.collider.time)

    @unittest.skip("Takes too long to execute")
    def test_mine_step_by_step(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        collider = ParticleColliderStepByStep(particles)

        collider.execute(timeout=100)
        self.assertEqual(648, len(collider.particles))


class ParticleColliderAnalytic:
    def __init__(self, particles):
        self.particles = list(particles)

    def execute(self):
        indexes = list(range(len(self.particles)))
        possible_collisions = self.calculate_possible_collisions()

        for time, collisions in possible_collisions.items():
            destroyed = set()
            for i, j in collisions:
                if i in indexes and j in indexes:
                    destroyed.add(i)
                    destroyed.add(j)
            for index in destroyed:
                indexes.remove(index)
        return len(indexes)

    def calculate_possible_collisions(self):
        possible_collisions = collections.defaultdict(list)
        for i, particle_i in enumerate(self.particles):
            for j, particle_j in enumerate(self.particles):
                if i >= j:
                    continue
                for collision_time in particle_i.collision_times_with(particle_j):
                    possible_collisions[collision_time].append((i, j))
        return possible_collisions


class TestParticleColliderAnalytic(unittest.TestCase):
    def test_mine_analytic(self):
        particles = Particle.gen_from_file("day_20_mine.txt")
        collider = ParticleColliderAnalytic(particles)

        self.assertEqual(648, collider.execute())
