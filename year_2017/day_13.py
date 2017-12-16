from unittest import TestCase, skip
import time
from aoc_utils import data_file
from multiprocessing.pool import Pool


def read_firewall(filename):
    with open(data_file(2017, filename)) as f:
        firewall = {}
        for line in f.readlines():
            depth, scan_range = map(int, line.split(": "))
            firewall[depth] = scan_range
        return firewall


def not_none(obj):
    return obj is not None


class FirewallBreaker:
    def __init__(self, filename):
        self.firewall = read_firewall(filename)
        self.total_depth = max(self.firewall.keys(), default=0)
        self.scan_lengths = {depth: 2 * scan_range - 2
                             for depth, scan_range in self.firewall.items()}
        self.severities = {depth: depth * scan_range
                           for depth, scan_range in self.firewall.items()}

    def min_safe_delay(self):
        delay = 0
        while self.will_collide(delay):
            delay += 1
        return delay

    def min_safe_delay_parallel(self):
        num_processes = 4
        queue_size = num_processes * 16
        batch_size = 128
        processors = Pool(processes=num_processes)

        results = []
        delay_min = 0
        while True:
            if any(r.ready() and r.get() is not None for r in results):
                break
            results = [r for r in results if not r.ready() or r.get() is not None]

            for _ in range(queue_size - len(results)):
                batch = delay_min, delay_min + batch_size
                result = processors.apply_async(self.which_will_collide, batch)
                results.append(result)
                delay_min += batch_size
            time.sleep(0.01)

        for result in results:
            result.wait()

        return min(result.get() for result in results if result.get() is not None)

    def which_will_collide(self, delay_min, delay_max):
        return min((i for i in range(delay_min, delay_max) if not self.will_collide(i)),
                   default=None)

    def will_collide(self, delay):
        collisions = self.gen_collisions(delay)
        return any(map(not_none, collisions))

    def total_severity(self, delay):
        collisions = self.gen_collisions(delay)
        severities = map(self.severities.__getitem__, collisions)
        return sum(severities)

    def gen_collisions(self, delay):
        for depth in range(self.total_depth + 1):
            if self.has_scanner(depth) and self.collide(delay + depth, depth):
                yield depth

    def collide(self, tick, depth):
        return self.scanner_position(tick, depth) == 0

    def scanner_position(self, tick, depth):
        scan_max = self.firewall[depth] - 1
        scan_length = self.scan_lengths[depth]
        virtual_position = tick % scan_length
        if virtual_position <= scan_max:
            return virtual_position
        else:
            return 2 * scan_max - virtual_position

    def has_scanner(self, depth):
        return depth in self.firewall


class TestFirewallBreaker(TestCase):
    def test_read_firewall(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertEqual(4, len(fwb.firewall))
        self.assertEqual(3, fwb.firewall[0])
        self.assertEqual(2, fwb.firewall[1])
        self.assertEqual(4, fwb.firewall[4])
        self.assertEqual(4, fwb.firewall[6])

        self.assertEqual(6, fwb.total_depth)

    def test_scanner_position(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertEqual(0, fwb.scanner_position(0, 0))
        self.assertEqual(1, fwb.scanner_position(1, 0))
        self.assertEqual(2, fwb.scanner_position(2, 0))
        self.assertEqual(1, fwb.scanner_position(3, 0))
        self.assertEqual(0, fwb.scanner_position(4, 0))

        self.assertEqual(0, fwb.scanner_position(0, 1))
        self.assertEqual(1, fwb.scanner_position(1, 1))
        self.assertEqual(0, fwb.scanner_position(2, 1))
        self.assertEqual(1, fwb.scanner_position(3, 1))
        self.assertEqual(0, fwb.scanner_position(4, 1))

        self.assertEqual(0, fwb.scanner_position(0, 4))
        self.assertEqual(1, fwb.scanner_position(1, 4))
        self.assertEqual(2, fwb.scanner_position(2, 4))
        self.assertEqual(3, fwb.scanner_position(3, 4))
        self.assertEqual(2, fwb.scanner_position(4, 4))

    def test_collide(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertIs(True, fwb.collide(0, 0))
        self.assertIs(False, fwb.collide(1, 1))
        self.assertIs(False, fwb.collide(4, 4))
        self.assertIs(True, fwb.collide(6, 6))

    def test_severity(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertEqual(4, len(fwb.severities))
        self.assertEqual(0, fwb.severities[0])
        self.assertEqual(2, fwb.severities[1])
        self.assertEqual(16, fwb.severities[4])
        self.assertEqual(24, fwb.severities[6])

    def test_total_severity(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertEqual(24, fwb.total_severity(0))
        self.assertEqual(2, fwb.total_severity(1))
        self.assertEqual(16, fwb.total_severity(2))

    def test_severity_mine(self):
        fwb = FirewallBreaker("day_13_mine.txt")
        self.assertEqual(1476, fwb.total_severity(0))

    def test_find_safe_delay_example(self):
        fwb = FirewallBreaker("day_13_example.txt")
        self.assertEqual(10, fwb.min_safe_delay())
        self.assertEqual(10, fwb.min_safe_delay_parallel())

    @skip("Still taking too long")
    def test_find_safe_delay_mine(self):
        fwb = FirewallBreaker("day_13_mine.txt")
        self.assertEqual(3937334, fwb.min_safe_delay_parallel())
