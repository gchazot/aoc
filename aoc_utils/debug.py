from contextlib import contextmanager
from time import thread_time


@contextmanager
def log_thread_time(title):
    start = thread_time()
    yield
    elapsed = thread_time() - start
    print("Timed {0}: {1:>10.4f} ms".format(title, elapsed * 1000))
