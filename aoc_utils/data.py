import os
from contextlib import contextmanager


def data_file(year, filename):
    parent = os.path.dirname
    root_dir = parent(parent(os.path.normpath(__file__)))
    year_dir = "year_{}".format(year)
    return os.path.join(root_dir, year_dir, "data", filename)


@contextmanager
def open_data_file(year, filename):
    file_path = data_file(year, filename)
    with open(file_path) as fh:
        yield fh


def data_lines(year, filename):
    with open_data_file(year, filename) as fh:
        for line in fh.readlines():
            yield line.strip('\n')


def data_text(year, filename):
    with open_data_file(year, filename) as fh:
        return fh.read()
