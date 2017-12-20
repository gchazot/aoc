import unittest
from collections import defaultdict
from aoc_utils import data_file


class Dancers:
    def __init__(self, num_dancers):
        self.size = num_dancers
        self.positions = list(range(self.size))
        self.offset = 0

    def order(self):
        offset_ed = [self.real_position(pos) for pos in self.positions]
        ranked = list(enumerate(offset_ed))
        ordered = sorted(ranked, key=lambda pair: pair[1])
        return "".join(map(lambda pair: self.program_name(pair[0]), ordered))

    def real_position(self, position):
        return (position + self.offset) % self.size

    def spin(self, count):
        self.offset += count

    def exchange(self, a, b):
        real_a = (a - self.offset) % self.size
        real_b = (b - self.offset) % self.size
        for i, pos_i in enumerate(self.positions):
            if pos_i == real_a:
                self.positions[i] = real_b
            elif pos_i == real_b:
                self.positions[i] = real_a

    def partners(self, a, b):
        a_num = self.program_number(a)
        b_num = self.program_number(b)
        a_pos = self.positions[a_num]
        self.positions[a_num] = self.positions[b_num]
        self.positions[b_num] = a_pos

    @staticmethod
    def program_name(program_number):
        return chr(ord('a') + program_number)

    @staticmethod
    def program_number(program_name):
        return ord(program_name) - ord('a')


class TestDancers(unittest.TestCase):
    def test_initial_position(self):
        dancers5 = Dancers(5)
        self.assertEqual("abcde", dancers5.order())
        dancers16 = Dancers(16)
        self.assertEqual("abcdefghijklmnop", dancers16.order())

    def test_spin(self):
        dancers5 = Dancers(5)
        dancers5.spin(1)
        self.assertEqual("eabcd", dancers5.order())
        dancers5.spin(1)
        self.assertEqual("deabc", dancers5.order())
        dancers5.spin(2)
        self.assertEqual("bcdea", dancers5.order())

    def test_exchange(self):
        dancers5 = Dancers(5)
        dancers5.exchange(0, 4)
        self.assertEqual("ebcda", dancers5.order())
        dancers5.exchange(1, 2)
        self.assertEqual("ecbda", dancers5.order())

    def test_partner(self):
        dancers5 = Dancers(5)
        dancers5.partners('a', 'b')
        self.assertEqual('bacde', dancers5.order())
        dancers5.partners('e', 'c')
        self.assertEqual('baedc', dancers5.order())


class DanceMaster:
    def __init__(self, filename):
        self.dance_plan = data_file(2017, filename)
        self.moves = self.gen_moves()
        self.moves_per_iteration = len(self.moves)
        self.num_iterations = None
        self.iteration = None
        self.i = None

    def dance_once(self, dancers):
        self.dance(dancers, 1)

    def dance(self, dancers, iterations):
        self.num_iterations = iterations
        self.iteration = 0
        while self.iteration < self.num_iterations:
            self.i = 0
            while self.i < self.moves_per_iteration:
                self.dance_one_move(dancers)
                self.i += 1
            self.iteration += 1

    def dance_one_move(self, dancers):
        move_text = self.moves[self.i]
        self.perform_move(dancers, move_text)

    def perform_move(self, dancers, move_text):
        if move_text[0] == 's':
            dancers.spin(int(move_text[1:]))
        elif move_text[0] == 'x':
            pair = move_text[1:].split('/')
            dancers.exchange(int(pair[0]), int(pair[1]))
        elif move_text[0] == 'p':
            pair = move_text[1:].split('/')
            dancers.partners(pair[0], pair[1])

    def gen_moves(self):
        with open(self.dance_plan) as plan:
            moves_text = plan.read()
            moves = moves_text.split(',')
        return moves


class SkippingDanceMaster(DanceMaster):
    def __init__(self, filename):
        super(self.__class__, self).__init__(filename)
        self.seen = defaultdict(lambda: (self.iteration, self.i))

    def dance_one_move(self, dancers):
        super(self.__class__, self).dance_one_move(dancers)

        order = dancers.order()
        seen = self.seen[order]
        if seen[1] == self.i and seen[0] != self.iteration:
            skip_iterations = self.iteration - seen[0]
            remaining_iterations = self.num_iterations - self.iteration
            total_skip = int(remaining_iterations / skip_iterations) * skip_iterations
            self.iteration += total_skip


class TestDanceMaster(unittest.TestCase):
    def test_example_once(self):
        master = DanceMaster("day_16_example.txt")
        dancers = Dancers(5)
        master.dance_once(dancers)
        self.assertEqual("baedc", dancers.order())

    def test_example_twice(self):
        master = DanceMaster("day_16_example.txt")
        dancers = Dancers(5)
        master.dance(dancers, 2)
        self.assertEqual("ceadb", dancers.order())

    def test_mine_once(self):
        master = DanceMaster("day_16_mine.txt")
        dancers = Dancers(16)
        master.dance_once(dancers)
        self.assertEqual("dcmlhejnifpokgba", dancers.order())

    @unittest.skip("Slightly too long")
    def test_mine_skipping(self):
        master = SkippingDanceMaster("day_16_mine.txt")
        dancers = Dancers(16)
        master.dance(dancers, 1000000000)
        self.assertEqual("ifocbejpdnklamhg", dancers.order())
