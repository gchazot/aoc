import collections
import datetime
import re
import unittest

from aoc_utils import data_lines


class TestGuardsJournal(unittest.TestCase):
    def test_parse_entry(self):
        def check(expected_day, expected_hour, expected_minute, expected_status, entry):
            expected_time = datetime.datetime(1518, 11, expected_day, expected_hour, expected_minute)
            expected = (expected_time, expected_status)
            self.assertEqual(expected, GuardsJournal.parse_entry(entry))

        check(1, 0, 0, "10", "[1518-11-01 00:00] Guard #10 begins shift")
        check(1, 0, 5, False, "[1518-11-01 00:05] falls asleep")
        check(1, 0, 25, True, "[1518-11-01 00:25] wakes up")
        check(1, 0, 30, False, "[1518-11-01 00:30] falls asleep")
        check(1, 0, 55, True, "[1518-11-01 00:55] wakes up")
        check(1, 23, 58, "99", "[1518-11-01 23:58] Guard #99 begins shift")
        check(2, 0, 40, False, "[1518-11-02 00:40] falls asleep")
        check(2, 0, 50, True, "[1518-11-02 00:50] wakes up")
        check(3, 0, 5, "10", "[1518-11-03 00:05] Guard #10 begins shift")
        check(3, 0, 24, False, "[1518-11-03 00:24] falls asleep")
        check(3, 0, 29, True, "[1518-11-03 00:29] wakes up")
        check(4, 0, 2, "99", "[1518-11-04 00:02] Guard #99 begins shift")
        check(4, 0, 36, False, "[1518-11-04 00:36] falls asleep")
        check(4, 0, 46, True, "[1518-11-04 00:46] wakes up")
        check(5, 0, 3, "99", "[1518-11-05 00:03] Guard #99 begins shift")
        check(5, 0, 45, False, "[1518-11-05 00:45] falls asleep")
        check(5, 0, 55, True, "[1518-11-05 00:55] wakes up")

    def test_starts_empty(self):
        journal = GuardsJournal()
        self.assertEqual(0, journal.size())

    def test_add_entry(self):
        journal = GuardsJournal()
        journal.add("[1518-11-01 00:00] Guard #10 begins shift")
        self.assertEqual(1, journal.size())
        journal.add("[1518-11-01 00:05] falls asleep")
        self.assertEqual(2, journal.size())
        journal.add("[1518-11-01 00:25] wakes up")
        self.assertEqual(3, journal.size())

        myjournal = GuardsJournal()
        for entry_line in data_lines(2018, "day_04_mine.txt"):
            myjournal.add(entry_line.strip())
        self.assertEqual(1097, myjournal.size())

    def test_get_entries_ordered(self):
        journal_preordered = GuardsJournal()
        journal_preordered.add("[1518-11-01 00:00] Guard #10 begins shift")
        journal_preordered.add("[1518-11-01 00:05] falls asleep")
        journal_preordered.add("[1518-11-01 00:25] wakes up")

        journal_unordered = GuardsJournal()
        journal_unordered.add("[1518-11-01 00:05] falls asleep")
        journal_unordered.add("[1518-11-01 00:00] Guard #10 begins shift")
        journal_unordered.add("[1518-11-01 00:25] wakes up")

        unordered_ordered = journal_unordered._get_entries_ordered()
        self.assertTrue(unordered_ordered[0] <= unordered_ordered[1])
        self.assertTrue(unordered_ordered[1] <= unordered_ordered[2])
        self.assertListEqual(
            journal_preordered._get_entries_ordered(),
            unordered_ordered,
        )

    def _add_simple_entries(self, journal):
        journal.add("[1518-11-01 00:00] Guard #1 begins shift")
        journal.add("[1518-11-01 00:01] falls asleep")
        journal.add("[1518-11-01 00:11] wakes up")
        journal.add("[1518-11-02 00:00] Guard #2 begins shift")
        journal.add("[1518-11-02 00:02] falls asleep")
        journal.add("[1518-11-02 00:22] wakes up")

    def test_get_guard_entries(self):
        journal = GuardsJournal()
        self._add_simple_entries(journal)

        guard_entries = journal._get_guard_entries()
        self.assertListEqual(["1", "2"], list(guard_entries.keys()))
        self.assertListEqual([
            (datetime.datetime(1518, 11, 1, 0, 1), False),
            (datetime.datetime(1518, 11, 1, 0, 11), True),
        ], list(guard_entries["1"]))

    def test_get_minute_histogram(self):
        journal = GuardsJournal()
        self._add_simple_entries(journal)

        def check_range(guard, start_minute, stop_minute, count):
            histograms = journal._get_minute_histograms()
            for minute in range(start_minute, stop_minute):
                self.assertEqual(count, histograms[guard].get(minute, 0))

        check_range("1", 0, 1, 0)
        check_range("1", 1, 11, 1)
        check_range("1", 11, 60, 0)
        check_range("2", 0, 2, 0)
        check_range("2", 2, 22, 1)
        check_range("2", 22, 60, 0)

        journal.add("[1518-11-11 00:00] Guard #1 begins shift")
        journal.add("[1518-11-11 00:10] falls asleep")
        journal.add("[1518-11-11 00:31] wakes up")

        check_range("1", 0, 1, 0)
        check_range("1", 1, 10, 1)
        check_range("1", 10, 11, 2)
        check_range("1", 11, 31, 1)
        check_range("1", 31, 60, 0)

    def _add_more_entries(self, journal):
        journal.add("[1518-11-11 00:00] Guard #1 begins shift")
        journal.add("[1518-11-11 00:10] falls asleep")
        journal.add("[1518-11-11 00:31] wakes up")
        journal.add("[1518-11-22 00:00] Guard #2 begins shift")
        journal.add("[1518-11-22 00:12] falls asleep")
        journal.add("[1518-11-22 00:13] wakes up")
        journal.add("[1518-11-23 00:00] Guard #2 begins shift")
        journal.add("[1518-11-23 00:12] falls asleep")
        journal.add("[1518-11-23 00:13] wakes up")

    def test_find_best_guard_best_minute(self):
        journal = GuardsJournal()
        self._add_simple_entries(journal)
        self._add_more_entries(journal)

        self.assertEqual(("1", 31, 10), journal.find_best_guard_best_minute())

        myjournal = GuardsJournal()
        for entry_line in data_lines(2018, "day_04_mine.txt"):
            myjournal.add(entry_line.strip())
        self.assertEqual(("727", 449, 49), myjournal.find_best_guard_best_minute())
        self.assertEqual(35623, 727 * 49)

    def test_find_best_minute_guard(self):
        journal = GuardsJournal()
        self._add_simple_entries(journal)
        self._add_more_entries(journal)

        self.assertEqual(("2", 12, 3), journal.find_best_minute_guard())

        myjournal = GuardsJournal()
        for entry_line in data_lines(2018, "day_04_mine.txt"):
            myjournal.add(entry_line.strip())
        self.assertEqual(("1097", 21, 15), myjournal.find_best_minute_guard())
        self.assertEqual(23037, 1097 * 21)

    def test_demo(self):
        journal = GuardsJournal()
        journal.add("[1518-11-01 00:00] Guard #10 begins shift")
        journal.add("[1518-11-01 00:05] falls asleep")
        journal.add("[1518-11-01 00:25] wakes up")
        journal.add("[1518-11-01 00:30] falls asleep")
        journal.add("[1518-11-01 00:55] wakes up")
        journal.add("[1518-11-02 00:40] falls asleep")
        journal.add("[1518-11-02 00:50] wakes up")
        journal.add("[1518-11-01 23:58] Guard #99 begins shift")
        journal.add("[1518-11-03 00:05] Guard #10 begins shift")
        journal.add("[1518-11-03 00:24] falls asleep")
        journal.add("[1518-11-03 00:29] wakes up")
        journal.add("[1518-11-04 00:02] Guard #99 begins shift")
        journal.add("[1518-11-04 00:36] falls asleep")
        journal.add("[1518-11-04 00:46] wakes up")
        journal.add("[1518-11-05 00:03] Guard #99 begins shift")
        journal.add("[1518-11-05 00:45] falls asleep")
        journal.add("[1518-11-05 00:55] wakes up")

        self.assertEqual(("10", 50, 24), journal.find_best_guard_best_minute())
        self.assertEqual(("99", 45, 3), journal.find_best_minute_guard())


class GuardsJournal:
    def __init__(self):
        self._entries = []

    def size(self):
        return len(self._entries)

    def add(self, entry_line):
        self._entries.append(entry_line)

    def find_best_guard_best_minute(self):
        guards_histograms = self._get_minute_histograms()

        total_sleeps = {guard: sum(counts.values()) for guard, counts in guards_histograms.items()}
        best_guard = max(total_sleeps.items(), key=lambda guard_sleep: guard_sleep[1])[0]
        guard_counts = guards_histograms[best_guard]
        best_minute = max(guard_counts.items(), key=lambda minute_count: minute_count[1])[0]

        return best_guard, total_sleeps[best_guard], best_minute

    def find_best_minute_guard(self):
        guards_histograms = self._get_minute_histograms()

        guard_bests = {
            guard: max(histogram.items(), key=lambda minute_count: minute_count[1])
            for guard, histogram in guards_histograms.items()
        }
        guard, (minute, count) = max(guard_bests.items(),
                                     key=lambda guard_minute_count: guard_minute_count[1][1])

        return guard, minute, count

    def _get_minute_histograms(self):
        guard_entries = self._get_guard_entries()
        result = dict()

        for guard, entries in guard_entries.items():
            asleep_pairs = []
            for time, status in entries:
                if not status:
                    sleep_time = time
                else:
                    asleep_pairs.append((sleep_time, time))

            guard_histogram = collections.defaultdict(lambda: 0)
            for start, stop in asleep_pairs:
                current_time = start
                while current_time < stop:
                    if current_time.hour == 0:
                        guard_histogram[current_time.minute] += 1
                    current_time += datetime.timedelta(minutes=1)

            result[guard] = guard_histogram
        return result

    def _get_guard_entries(self):
        entries = self._get_entries_ordered()
        result = collections.defaultdict(list)
        guard = None
        for entry in entries:
            time, status = self.parse_entry(entry)
            if isinstance(status, str):
                guard = status
            else:
                assert(isinstance(status, bool))
                result[guard].append((time, status))
        return result

    def _get_entries_ordered(self):
        return sorted(self._entries)

    ENTRY_PATTERN = re.compile(r"^\[(?P<time>[^]]+)\] (?P<text>.+)$")

    @staticmethod
    def parse_entry(entry_line):
        match = GuardsJournal.ENTRY_PATTERN.match(entry_line)
        assert(match is not None)

        time_str = match.group("time")
        time = datetime.datetime.strptime(time_str, "%Y-%m-%d %H:%M")
        text = match.group("text")

        if text.startswith("Guard"):
            offset_start = len("Guard #")
            number_str = text[offset_start:].split()[0]
            return time, number_str
        elif text.startswith("wakes"):
            return time, True
        elif text.startswith("falls"):
            return time, False
