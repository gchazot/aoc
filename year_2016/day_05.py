from itertools import islice, takewhile
from unittest import TestCase, skip
import hashlib


def generate_password_characters(door_id):
    i = 0
    n = 0
    hash_function_base = hashlib.md5(door_id.encode("utf-8"))
    while True:
        hash_function = hash_function_base.copy()
        hash_function.update(str(i).encode("utf-8"))
        md5_hash = hash_function.hexdigest()
        if md5_hash[:5] == "00000":
            yield md5_hash[5], md5_hash[6]
            n += 1
        i += 1


def get_door_1_password(door_id, length):
    entries = generate_password_characters(door_id)
    password_chars = map(lambda entry: entry[0], islice(entries, length))
    return "".join(password_chars)


def get_door_2_password(door_id, length):
    entries = generate_password_characters(door_id)
    searching_for = set(map(str, range(length)))
    password_chars = ["_" for _ in range(length)]
    for position, value in takewhile(lambda _: len(searching_for) > 0, entries):
        if position in searching_for:
            password_chars[int(position, base=16)] = value
            searching_for.remove(position)
    return "".join(password_chars)


class TestDoorPassword(TestCase):
    @skip("Too slow")
    def test_1_example(self):
        self.assertEqual("18f47a30", get_door_1_password("abc", 8))

    @skip("Too slow")
    def test_1_mine(self):
        self.assertEqual("4543c154", get_door_1_password("ojvtpuvg", 8))

    @skip("Too slow")
    def test_2_example(self):
        self.assertEqual("05ace8e3", get_door_2_password("abc", 8))

    @skip("Too slow")
    def test_2_mine(self):
        self.assertEqual("1050cbbd", get_door_2_password("ojvtpuvg", 8))
