from unittest import TestCase
from aoc_utils import data_file


def check_supports_tls(ipv7):
    members = split_brackets(ipv7)
    in_brackets = False
    found_out_brackets = False
    for member in members:
        if check_member_has_abba(member):
            if in_brackets:
                return False
            else:
                found_out_brackets = True
        in_brackets = not in_brackets
    return found_out_brackets


def gen_aba(member):
    for i in range(len(member) - 2):
        sub_member = member[i:i + 3]
        if is_aba_or_abba(sub_member):
            yield sub_member


def get_aba_in_out_brackets(ipv7):
    members = split_brackets(ipv7)
    in_brackets = False
    found_in_brackets = []
    found_out_brackets = []
    for member in members:
        if in_brackets:
            found_in_brackets += gen_aba(member)
        else:
            found_out_brackets += gen_aba(member)
        in_brackets = not in_brackets
    return found_in_brackets, found_out_brackets


def make_bab(aba):
    aba_list = list(aba)
    return "".join((aba_list[1], aba_list[0], aba_list[1]))


def check_supports_ssl(ipv7):
    found_in_brackets, found_out_brackets = get_aba_in_out_brackets(ipv7)
    babs_in_brackets = map(make_bab, found_in_brackets)
    unique_abas_out_brackets = set(found_out_brackets)
    unique_babs_in_brackets = set(babs_in_brackets)
    aba_and_bab = unique_babs_in_brackets.intersection(unique_abas_out_brackets)
    return len(aba_and_bab) > 0


def is_aba_or_abba(chars):
    return (len(chars) >= 3 and
            chars[0] != chars[1] and
            chars[0] == chars[-1] and
            chars[1] == chars[-2])


def split_brackets(ipv7):
    return ipv7.replace("]", "[").split("[")


def check_member_has_abba(member):
    return any(map(is_aba_or_abba, (member[i:i + 4] for i in range(len(member) - 3))))


def test_all_ips(filename, check_function):
    with open(data_file(2016, filename)) as f:
        valid_ips = map(check_function, f.readlines())
        return sum(valid_ips)


class TestIPv7(TestCase):
    def test_is_abba(self):
        self.assertTrue(is_aba_or_abba(list("abba")))
        self.assertTrue(is_aba_or_abba(list("baab")))
        self.assertTrue(is_aba_or_abba(list("xyyx")))
        self.assertTrue(is_aba_or_abba(list("yzzy")))

        self.assertFalse(is_aba_or_abba(list("x")))
        self.assertFalse(is_aba_or_abba(list("xx")))
        self.assertFalse(is_aba_or_abba(list("xxx")))
        self.assertFalse(is_aba_or_abba(list("xxxx")))
        self.assertFalse(is_aba_or_abba(list("xxxxx")))
        self.assertFalse(is_aba_or_abba(list("xyyxx")))
        self.assertFalse(is_aba_or_abba(list("xxyyx")))

        self.assertFalse(is_aba_or_abba(list("xxyy")))
        self.assertFalse(is_aba_or_abba(list("xyxy")))
        self.assertFalse(is_aba_or_abba(list("yxyx")))
        self.assertFalse(is_aba_or_abba(list("xxyy")))

    def test_split_brackets(self):
        self.assertEqual(3, len(split_brackets("abba[mnop]qrst")))

    def test_check_member(self):
        self.assertTrue(check_member_has_abba("abba"))
        self.assertFalse(check_member_has_abba("abab"))
        self.assertTrue(check_member_has_abba("abbab"))
        self.assertTrue(check_member_has_abba("babba"))
        self.assertTrue(check_member_has_abba("abababababba"))
        self.assertFalse(check_member_has_abba("abxba"))
        self.assertFalse(check_member_has_abba("aba"))
        self.assertFalse(check_member_has_abba("abbbbbba"))

    def test_check_ip_for_tls(self):
        self.assertTrue(check_supports_tls("abba[mnop]qrst"))
        self.assertFalse(check_supports_tls("abcd[bddb]xyyx"))
        self.assertFalse(check_supports_tls("aaaa[qwer]tyui"))
        self.assertTrue(check_supports_tls("ioxxoj[asdfgh]zxcvbn"))

    def test_all_ips_for_tls_mine(self):
        self.assertEqual(115, test_all_ips("day_07_mine.txt", check_supports_tls))

    def test_check_ip_for_ssl(self):
        self.assertTrue(check_supports_ssl("aba[bab]xyz"))
        self.assertFalse(check_supports_ssl("xyx[xyx]xyx"))
        self.assertTrue(check_supports_ssl("aaa[kek]eke"))
        self.assertTrue(check_supports_ssl("zazbz[bzb]cdb"))

    def test_all_ips_for_tls_mine(self):
        self.assertEqual(231, test_all_ips("day_07_mine.txt", check_supports_ssl))