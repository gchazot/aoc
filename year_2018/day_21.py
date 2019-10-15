import unittest
from aoc_utils.data import data_lines
from year_2018.day_19 import JumpingProcessor, Program


decompiled_program = '''
 0:    E = 123
 1:    E = 456 & E
 2:    E = E == 72
 3:    A = A + E
 4:    A = 0 -> GOTO 0
 5:    E = 0
 6:    C = E | 65536
 7:    E = 521363
 8:    D = C & 255
 9:    E = D + E
10:    E = E & 16777215
11:    E = E * 65899
12:    E = E & 16777215
13:    D = 256 > C
14:    A = A + D
15:    A = A + 1  -> SKIP
16:    A = 27 -> GOTO 27
17:    D = 0
18:    B = D + 1
19:    B = B * 256
20:    B = B > C
21:    A = A + B
22:    A = A + 1   -> SKIP
23:    A = 25  -> GOTO 25
24:    D = D + 1
25:    A = 17  -> GOTO 17
26:    C = D
27:    A = 7   -> GOTO 7
28:    D = E == X
29:    A = A + D  -> Terminate
30:    A = 5   -> GOTO 5
'''


def decompiled(x):
    e = 0
    while e != x:
        e = decompiled_loop_1(e)


def decompiled_loop_1(e):
    c = e | 65536
    e = 521363
    while True:
        d = c & 255
        e = e + d
        e = e & 16777215
        e = e * 65899
        e = e & 16777215
        if c < 256:
            break
        d = 0
        while (d + 1) * 256 <= c:
            d = d + 1
        c = d
    return e


def find_first_before_looping_over():
    seen = set()
    e = 0
    while True:
        next_e = decompiled_loop_1(e)
        if next_e in seen:
            break
        seen.add(next_e)
        e = next_e
    return e


class TestChronalConversion(unittest.TestCase):
    def test_quickest_stop(self):
        decompiled(1024276)

    @unittest.skip("Too slow")
    def test_slowest_stop(self):
        self.assertEqual(5876609, find_first_before_looping_over())

    def test_execute_program(self):
        program = Program(
            num_registers=6,
            input_lines=data_lines(2018, "day_21_mine.txt"),
            initial_registers=[1024276, 0, 0, 0, 0, 0],
            processor_class=JumpingProcessor,
        )
        program.execute(10000, log=False)

        self.assertEqual(1848, program.total_instructions)