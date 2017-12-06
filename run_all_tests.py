import unittest
import os


def discover(path):
    loader = unittest.TestLoader()
    return loader.discover(path, pattern="day_*.py")


def run_all(path):
    tests = discover(path)
    test_runner = unittest.TextTestRunner()
    test_runner.run(tests)


if __name__ == "__main__":
    run_all(os.path.dirname(__file__))
