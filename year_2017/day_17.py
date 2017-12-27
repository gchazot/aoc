import unittest


class CircularBuffer:
    def __init__(self):
        self.list = [0]
        self.position = 0

    def insert(self, value):
        self.position += 1
        self.list.insert(self.position, value)

    def as_list(self):
        return self.list

    def step(self, steps):
        self.position = self._offset_position(steps)

    def value_at(self, offset):
        return self.list[self._offset_position(offset)]

    def _offset_position(self, offset):
        return (self.position + offset) % self._buffer_length()

    def _buffer_length(self):
        return len(self.list)


class TestCircularBuffer(unittest.TestCase):
    def setUp(self):
        self.buffer = CircularBuffer()

    def test_initial_configuration(self):
        self.assertListEqual([0], self.buffer.as_list())

    def test_inserts_value_after_position(self):
        self.buffer.insert(3)
        self.assertListEqual([0, 3], self.buffer.as_list())
        self.buffer.insert(-9)
        self.assertListEqual([0, 3, -9], self.buffer.as_list())

    def test_initial_position(self):
        self.assertEqual(0, self.buffer.position)
        self.buffer.insert(3)
        self.assertEqual(1, self.buffer.position)
        self.buffer.insert(-9)
        self.assertEqual(2, self.buffer.position)

    def test_step_forward_after_insert(self):
        self.buffer.insert(3)
        self.assertEqual(1, self.buffer.position)

        self.buffer.insert(-1)
        self.assertEqual(2, self.buffer.position)

        self.buffer.insert(12)
        self.assertEqual(3, self.buffer.position)

    def test_step_forward(self):
        self.buffer.step(1)
        self.assertEqual(0, self.buffer.position)

        self.buffer.step(2)
        self.assertEqual(0, self.buffer.position)

        self.buffer.insert(3)
        self.buffer.insert(-1)
        self.buffer.insert(12)

        self.assertEqual(3, self.buffer.position)
        self.buffer.step(1)
        self.assertEqual(0, self.buffer.position)
        self.buffer.step(2)
        self.assertEqual(2, self.buffer.position)
        self.buffer.step(3)
        self.assertEqual(1, self.buffer.position)

    def test_value_at_position(self):
        self.buffer.insert(3)
        self.buffer.insert(-1)
        self.buffer.insert(12)

        self.assertEqual(12, self.buffer.value_at(0))
        self.assertEqual(0, self.buffer.value_at(1))
        self.assertEqual(3, self.buffer.value_at(2))
        self.assertEqual(-1, self.buffer.value_at(3))


class SpinStorm:
    def __init__(self, buffer=None):
        self.buffer = buffer or CircularBuffer()
        self.next_value = 1

    def walk(self, repetitions, steps):
        for _ in range(repetitions):
            self.step(steps)

    def step(self, steps):
        self.buffer.step(steps)
        self.buffer.insert(self.next_value)
        self.next_value += 1


class TestSpinStorm(unittest.TestCase):
    def setUp(self):
        self.buffer = CircularBuffer()
        self.storm = SpinStorm(self.buffer)

    def test_initial_state(self):
        self.assertListEqual([0], self.buffer.as_list())
        self.assertEqual(self.buffer, self.storm.buffer)

    def test_single_step_example(self):
        self.storm.step(3)
        self.assertListEqual([0, 1], self.buffer.as_list())
        self.assertEqual(1, self.buffer.position)

        self.storm.step(3)
        self.assertListEqual([0, 2, 1], self.buffer.as_list())
        self.assertEqual(1, self.buffer.position)

        self.storm.step(3)
        self.storm.step(3)
        self.storm.step(3)
        self.storm.step(3)
        self.assertListEqual([0, 5, 2, 4, 3, 6, 1], self.buffer.as_list())
        self.assertEqual(5, self.buffer.position)

        self.storm.step(3)
        self.storm.step(3)
        self.storm.step(3)
        self.assertListEqual([0, 9, 5, 7, 2, 4, 3, 8, 6, 1], self.buffer.as_list())
        self.assertEqual(1, self.buffer.position)

    def test_many_steps_example(self):
        self.storm.walk(9, 3)
        self.assertListEqual([0, 9, 5, 7, 2, 4, 3, 8, 6, 1], self.buffer.as_list())
        self.assertEqual(1, self.buffer.position)

        self.storm.walk(2017 - 9, 3)
        self.assertEqual(2017, self.buffer.value_at(0))
        self.assertEqual(638, self.buffer.value_at(1))

    def test_many_steps_mine(self):
        self.storm.walk(2017, 304)
        self.assertEqual(2017, self.buffer.value_at(0))
        self.assertEqual(1173, self.buffer.value_at(1))

    @unittest.skip("Wayyyy too long")
    def test_angry_steps_mine(self):
        iterations = 50000000
        self.storm.walk(iterations, 304)
        self.assertEqual(iterations, self.buffer.value_at(0))
        self.assertEqual(1173, self.buffer.as_list()[1])


class CheatBuffer(CircularBuffer):
    def __init__(self):
        self.position = 0
        self.length = 1
        self.value_at_position_one = None

    def insert(self, value):
        self.position += 1
        self.length += 1
        if self.position == 1:
            self.value_at_position_one = value

    def value_at_one(self):
        return self.value_at_position_one

    def _buffer_length(self):
        return self.length


class TestCheatBufferStorm(unittest.TestCase):
    def setUp(self):
        self.buffer = CheatBuffer()
        self.storm = SpinStorm(self.buffer)

    def test_initial_value(self):
        self.assertEqual(None, self.buffer.value_at_one())

    def test_value_after_steps_example(self):
        self.storm.step(3)
        self.assertEqual(1, self.buffer.value_at_one())

        self.storm.step(3)
        self.assertEqual(2, self.buffer.value_at_one())
        self.storm.step(3)
        self.storm.step(3)
        self.assertEqual(2, self.buffer.value_at_one())

        self.storm.step(3)
        self.assertEqual(5, self.buffer.value_at_one())
        self.storm.step(3)
        self.storm.step(3)
        self.storm.step(3)
        self.assertEqual(5, self.buffer.value_at_one())

        self.storm.step(3)
        self.assertEqual(9, self.buffer.value_at_one())

    @unittest.skip("Still too long but reasonable")
    def test_angry_steps_mine(self):
        iterations = 50000000
        self.storm.walk(iterations, 304)
        self.assertEqual(1930815, self.buffer.value_at_one())
