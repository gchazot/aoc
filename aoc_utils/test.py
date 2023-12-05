from functools import partial
from platform import python_implementation
from unittest import skipIf

pypy_only = partial(skipIf, python_implementation() != "PyPy")
