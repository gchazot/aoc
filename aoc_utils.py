import os


def data_file(year, filename):
    root_dir = os.path.dirname(os.path.normpath(__file__))
    year_dir = "year_{}".format(year)
    return os.path.join(root_dir, year_dir, "data", filename)
