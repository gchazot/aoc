import json
import unittest


class TestFuelGrid(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super(TestFuelGrid, self).__init__(*args, **kwargs)
        self._grids = {}

    def _grid(self, **kwargs):
        key = json.dumps(kwargs)
        return self._grids.get(key, FuelGrid(**kwargs))

    def test_cell_power(self):
        self.assertEqual(4, self._grid(serial_num=8).cell_power(3, 5))
        self.assertEqual(-5, self._grid(serial_num=57).cell_power(122, 79))
        self.assertEqual(0, self._grid(serial_num=39).cell_power(217, 196))
        self.assertEqual(4, self._grid(serial_num=71).cell_power(101, 153))

    def test_calculate_power_grid(self):
        grid8 = self._grid(serial_num=8, dimension=5)
        grid8.calculate_power_grid()
        self.assertEqual(4, grid8.power_at(3, 5))

        grid18 = self._grid(serial_num=18)
        grid18.calculate_power_grid()
        self.assertEqual(4, grid18.power_at(33, 45))
        self.assertEqual(4, grid18.power_at(34, 45))
        self.assertEqual(4, grid18.power_at(35, 45))
        self.assertEqual(3, grid18.power_at(33, 46))
        self.assertEqual(3, grid18.power_at(34, 46))
        self.assertEqual(4, grid18.power_at(35, 46))
        self.assertEqual(1, grid18.power_at(33, 47))
        self.assertEqual(2, grid18.power_at(34, 47))
        self.assertEqual(4, grid18.power_at(35, 47))

    def test_calculate_fuel_squares(self):
        grid18 = self._grid(serial_num=18)
        squares_18_3 = grid18.calculate_fuel_squares(square_size=3)

        self.assertEqual(29, squares_18_3[33-1][45-1])

        grid42 = self._grid(serial_num=42)
        squares_42_3 = grid42.calculate_fuel_squares(square_size=3)

        self.assertEqual(30, squares_42_3[21-1][61-1])

    def test_find_best_square(self):
        grid18 = self._grid(serial_num=18)
        self.assertEqual(((33, 45), 3), grid18.find_best_square(square_sizes=[3]))

    @unittest.skip("too slow")
    def test_find_best_square_with_size(self):
        grid18 = self._grid(serial_num=18)
        self.assertEqual(((90, 269), 16), grid18.find_best_square(square_sizes=range(1, 301)))

    def test_find_best_square_mine(self):
        grid_mine = self._grid(serial_num=1955)
        self.assertEqual(((21, 93), 3), grid_mine.find_best_square(square_sizes=[3]))

    @unittest.skip("too slow")
    def test_find_best_square_mine_with_size(self):
        grid_mine = self._grid(serial_num=1955)
        self.assertEqual(((231, 108), 14), grid_mine.find_best_square(square_sizes=range(1, 301)))


class FuelGrid:
    def __init__(self, serial_num, dimension=300):
        self._serial = serial_num
        self._cells_power = None
        self._square_powers = {}
        self._dimension = dimension

    def power_at(self, x, y):
        return self._cells_power[x-1][y-1]

    def find_best_square(self, square_sizes):
        best_size = None
        best_coords = None
        best_square = None
        for square_size in square_sizes:
            squares = self.calculate_fuel_squares(square_size)

            for x in self._square_grid_range(square_size):
                for y in self._square_grid_range(square_size):
                    square = squares[x][y]
                    if best_square is None or square > best_square:
                        best_square = square
                        best_size = square_size
                        best_coords = x+1, y+1

        return best_coords, best_size

    def calculate_fuel_squares(self, square_size):
        if square_size not in self._square_powers:
            self.calculate_power_grid()
            square_range = range(square_size)

            self._square_powers[square_size] = [
                [
                    sum(
                        self._cells_power[x + i][y + j]
                        for i in square_range for j in square_range
                    )
                    for y in self._square_grid_range(square_size)
                ]
                for x in self._square_grid_range(square_size)
            ]
        return self._square_powers[square_size]

    def calculate_power_grid(self):
        if self._cells_power is None:
            self._cells_power = [
                [self.cell_power(x+1, y+1) for y in range(self._dimension)]
                for x in range(self._dimension)
            ]

    def cell_power(self, x, y):
        rack_id = x + 10
        power_level = (rack_id * y + self._serial) * rack_id
        hundreds = int(power_level / 100) % 10
        return hundreds - 5

    def _square_grid_range(self, square_size):
        grid_dimension = self._dimension - square_size + 1
        return range(grid_dimension)
