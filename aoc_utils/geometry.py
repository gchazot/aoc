def add_coordinates(a, b):
    """Helper to add 2 coordinates vectors"""
    return tuple(u + v for u, v in zip(a, b))
