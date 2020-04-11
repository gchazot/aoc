import unittest
import os
import sys


def discover(path):
    loader = unittest.TestLoader()
    return loader.discover(path, pattern="*.py")


def run_all(path):
    tests = discover(path)
    test_runner = unittest.TextTestRunner()
    test_result = test_runner.run(tests)
    return test_result


if __name__ == "__main__":
    base_path = os.path.dirname(__file__)
    results = run_all(base_path)
    errors = len(results.errors)
    failures = len(results.failures)
    sys.exit(2 if errors else 1 if failures else 0)
