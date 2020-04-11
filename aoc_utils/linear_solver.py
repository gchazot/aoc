from __future__ import division

import unittest
from array import array


class TestLinearProblem(unittest.TestCase):
    def test_add_variable(self):
        pb = LinearProblem()

        pb.add_variable("unbound")
        self.assertEqual(1, len(pb.variables))
        self.assertEqual((None, None), pb.variables["unbound"])

        pb.add_variable("lower", lower_bound=42)
        self.assertEqual(2, len(pb.variables))
        self.assertEqual((42, None), pb.variables["lower"])

        pb.add_variable("upper", upper_bound=69)
        self.assertEqual(3, len(pb.variables))
        self.assertEqual((None, 69), pb.variables["upper"])

        pb.add_variable("bound", lower_bound=33, upper_bound=55)
        self.assertEqual(4, len(pb.variables))
        self.assertEqual((33, 55), pb.variables["bound"])

    def test_add_variable_rejects_duplicate(self):
        pb = LinearProblem()

        pb.add_variable("duplicate")
        self.assertEqual(1, len(pb.variables))

        self.assertRaises(RuntimeError, pb.add_variable, "duplicate")
        self.assertEqual(1, len(pb.variables))

    def test_add_constraint(self):
        pb = LinearProblem()
        variables = {
            "x": {"lower_bound": None, "upper_bound": None},
            "y": {"lower_bound": 33, "upper_bound": None},
            "z": {"lower_bound": None, "upper_bound": 42},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        pb.add_constraint({"x": 1}, lower_bound=2, upper_bound=3)
        pb.add_constraint({"y": 4}, lower_bound=5, upper_bound=None)
        pb.add_constraint({"z": 5}, lower_bound=None, upper_bound=7)

        pb.add_constraint({"x": 8, "y": 9, "z": 10}, lower_bound=11, upper_bound=12)

        self.assertRaises(RuntimeError, pb.add_constraint, {"x": 13})
        self.assertRaises(
            RuntimeError, pb.add_constraint, {"x": 14}, lower_bound=None, upper_bound=None,
        )
        self.assertRaises(
            RuntimeError, pb.add_constraint, {"x": 15}, lower_bound=17, upper_bound=16,
        )

        self.assertRaises(
            RuntimeError, pb.add_constraint, {}, lower_bound=18, upper_bound=19,
        )
        self.assertRaises(
            RuntimeError, pb.add_constraint, {"x": 0}, lower_bound=20, upper_bound=21,
        )
        self.assertRaises(
            RuntimeError, pb.add_constraint, {"x": None}, lower_bound=20, upper_bound=21,
        )

        self.assertRaises(
            RuntimeError, pb.add_constraint, {"unknown": 22}, lower_bound=23, upper_bound=24,
        )

    def test_set_objective(self):
        pb = LinearProblem()
        variables = {
            "x": {"lower_bound": None, "upper_bound": None},
            "y": {"lower_bound": 33, "upper_bound": None},
            "z": {"lower_bound": None, "upper_bound": 42},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 1})
        pb.set_objective(LinearProblem.MAXIMIZE, {"y": 2})
        pb.set_objective(LinearProblem.MAXIMIZE, {"z": 3})
        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 4, "y": 5, "z": 6})

        self.assertRaises(RuntimeError, pb.set_objective, LinearProblem.MAXIMIZE, {"unknown": 7})

        self.assertRaises(RuntimeError, pb.set_objective, LinearProblem.MAXIMIZE, {})
        self.assertRaises(RuntimeError, pb.set_objective, LinearProblem.MAXIMIZE, {"x": 0})
        self.assertRaises(RuntimeError, pb.set_objective, LinearProblem.MAXIMIZE, {"x": None})

    def test_example_a1(self):
        # http://fourier.eng.hmc.edu/e176/lectures/NM/node32.html
        pb = LinearProblem()

        variables = {
            "x": {"lower_bound": None, "upper_bound": None},
            "y": {"lower_bound": 0, "upper_bound": None},
            "z": {"lower_bound": 0, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"x": 1, "y": -2, "z": 1}, {"lower_bound": 3, "upper_bound": 3}),
            ({"x": 3, "y": -1, "z": 4}, {"lower_bound": 10, "upper_bound": 10}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 2, "y": -1, "z": 3})

        solution, objective = pb.solve()
        self.assertEqual(20/3, objective)

    def test_example_a2(self):
        # http://fourier.eng.hmc.edu/e176/lectures/NM/node32.html
        pb = LinearProblem()

        variables = {
            "x": {"lower_bound": 0, "upper_bound": None},
            "y": {"lower_bound": 0, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"x": 2, "y": 1}, {"lower_bound": None, "upper_bound": 18}),
            ({"x": 6, "y": 5}, {"lower_bound": None, "upper_bound": 60}),
            ({"x": 2, "y": 5}, {"lower_bound": None, "upper_bound": 40}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 2, "y": 3})

        solution, objective = pb.solve()
        self.assertEqual(28, objective)

    def test_example_a2_standard_form(self):
        # http://fourier.eng.hmc.edu/e176/lectures/NM/node32.html
        pb = LinearProblem()

        variables = {
            "x": {"lower_bound": 0, "upper_bound": None},
            "y": {"lower_bound": 0, "upper_bound": None},
            "u": {"lower_bound": 0, "upper_bound": None},
            "v": {"lower_bound": 0, "upper_bound": None},
            "w": {"lower_bound": 0, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"x": 2, "y": 1, "u": 1}, {"lower_bound": 18, "upper_bound": 18}),
            ({"x": 6, "y": 5, "v": 1}, {"lower_bound": 60, "upper_bound": 60}),
            ({"x": 2, "y": 5, "w": 1}, {"lower_bound": 40, "upper_bound": 40}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 2, "y": 3})

        solution, objective = pb.solve()
        self.assertEqual(28, objective)
        self.assertEqual({'x': 5.0, 'y': 6.0, 'u': 0, 'v': 0, 'w': 0}, solution)

    def test_example_b(self):
        # https://github.com/j2kun/simplex-algorithm/blob/237be434eac76f0711a2136b363b0ebc42095cca/simplex.py#L83
        pb = LinearProblem()

        variables = {
            "x": {"lower_bound": 0, "upper_bound": None},
            "y": {"lower_bound": 0, "upper_bound": None},
            "z": {"lower_bound": 0, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"x": 15, "y": 20, "z": 25}, {"lower_bound": None, "upper_bound": 1200}),
            ({"x": 35, "y": 60, "z": 60}, {"lower_bound": None, "upper_bound": 3000}),
            ({"x": 20, "y": 30, "z": 25}, {"lower_bound": None, "upper_bound": 1500}),
            ({"x": 0, "y": 250, "z": 0}, {"lower_bound": 500, "upper_bound": None}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 300, "y": 250, "z": 450})

        solution, objective = pb.solve()
        self.assertEqual(23060, objective)

    def test_example_c1(self):
        # http://people.brunel.ac.uk/~mastjjb/jeb/or/morelp.html
        pb = LinearProblem()

        variables = {
            "x": {"lower_bound": 45, "upper_bound": None},
            "y": {"lower_bound": 5, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"x": 50, "y": 24}, {"lower_bound": None, "upper_bound": 40*60}),
            ({"x": 30, "y": 33}, {"lower_bound": None, "upper_bound": 35*60}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MAXIMIZE, {"x": 1, "y": 1})

        solution, objective = pb.solve()
        self.assertEqual(50+1.25, objective)
        self.assertSetEqual({"x", "y"}, frozenset(solution.keys()))
        self.assertAlmostEqual(45, solution["x"])
        self.assertAlmostEqual(6.25, solution["y"])

    @unittest.skip("Only works for maximisation for now")
    def test_example_c5(self):
        # http://people.brunel.ac.uk/~mastjjb/jeb/or/morelp.html
        pb = LinearProblem()

        variables = {
            "a": {"lower_bound": 0, "upper_bound": None},
            "b": {"lower_bound": 0, "upper_bound": None},
            "c": {"lower_bound": 0, "upper_bound": None},
        }
        for name, bounds in variables.items():
            pb.add_variable(name, **bounds)

        constraints = [
            ({"a": 1, "b": 1}, {"lower_bound": 11, "upper_bound": None}),
            ({"a": 1, "b": -1}, {"lower_bound": None, "upper_bound": 5}),
            ({"a": -1, "b": -1, "c": 1}, {"lower_bound": 0, "upper_bound": 0}),
            ({"a": 7, "b": 12}, {"lower_bound": 35, "upper_bound": None}),
        ]
        for equation, bounds in constraints:
            pb.add_constraint(equation, **bounds)

        pb.set_objective(LinearProblem.MINIMIZE, {"a": 4, "b": 5, "c": 6})

        solution, objective = pb.solve()
        # print(solution, objective)
        self.assertEqual(113, objective)
        self.assertSetEqual({"a", "b", "c"}, frozenset(solution.keys()))
        self.assertAlmostEqual(8, solution["a"])
        self.assertAlmostEqual(3, solution["b"])
        self.assertAlmostEqual(11, solution["c"])


class LinearProblem:
    MAXIMIZE = 1
    MINIMIZE = -1

    def __init__(self):
        self.variables = {}  # name -> (lower_bound, upper_bound)
        self.constraints = []  # (equation, value) so that equation = value
        self.objective = None  # equation ({variable_name -> coefficient})
        self.optimization_direction = None  # MINIMIZE / MAXIMIZE
        self._slacks = {}  # constraint_index -> coefficient (+1/-1)

    def add_variable(self, name, lower_bound=None, upper_bound=None):
        if name in self.variables:
            raise RuntimeError("Duplicate variable {0}".format(name))
        self.variables[name] = (lower_bound, upper_bound)
        if lower_bound is not None or upper_bound is not None:
            self.add_constraint({name: 1}, lower_bound, upper_bound)

    def add_constraint(self, equation, lower_bound=None, upper_bound=None):
        if lower_bound is None and upper_bound is None:
            raise RuntimeError("Unbound equation")
        if lower_bound is not None and upper_bound is not None and lower_bound > upper_bound:
            raise RuntimeError("Impossible equation ({0} <= {1}".format(lower_bound, upper_bound))
        simplified_equation = self._clean_equation(equation)

        if lower_bound == upper_bound:
            self._add_constraint_equal(simplified_equation, lower_bound)
        if lower_bound:
            self._add_constraint_lower_bound(simplified_equation, lower_bound)
        if upper_bound:
            self._add_constraint_upper_bound(simplified_equation, upper_bound)

    def set_objective(self, direction, equation):
        if direction not in (self.MAXIMIZE, self.MINIMIZE):
            raise RuntimeError("Invalid optimization direction {0}".format(direction))
        self.optimization_direction = direction
        simplified_equation = self._clean_equation(equation)
        final_equation = {
            variable: coefficient * self.optimization_direction
            for variable, coefficient in simplified_equation.items()
        }
        self.objective = final_equation

    def solve(self):
        n, variable_factors = self._normalise_variables()
        m = len(self.constraints)
        tableau, variable_factors = self._build_tableau(n, m, variable_factors)

        while not tableau.is_optimal():
            pivot_j = tableau.find_pivot_column()
            if pivot_j is None:
                # print("Pivots: column", pivot_j)
                break

            pivot_i = tableau.find_pivot_row(pivot_j)
            if pivot_i is None or pivot_j is None:
                # print("Pivots: row", pivot_i, ", column", pivot_j)
                break

            tableau.pivot(pivot_i, pivot_j)

        primal_solution = dict(tableau.primal_solution())
        objective = tableau.get_objective_value()

        solution = {
            name: self.optimization_direction * sum(
                primal_solution.get(j, 0) * f for j, f in factors.items()
            )
            for name, factors in variable_factors.items()
        }
        return solution, objective

    def _add_constraint_equal(self, constraint, equal_to):
        self.constraints.append((constraint, equal_to))
        return len(self.constraints) - 1

    def _add_constraint_lower_bound(self, equation, greater_than):
        self._add_constraint_with_slack(equation, -1, greater_than)

    def _add_constraint_upper_bound(self, equation, less_than):
        self._add_constraint_with_slack(equation, 1, less_than)

    def _add_constraint_with_slack(self, equation, slack_coefficient, equal_to):
        constraint_index = self._add_constraint_equal(equation, equal_to)
        self._slacks[constraint_index] = slack_coefficient

    def _clean_equation(self, equation):
        if any(variable_name not in self.variables for variable_name in equation.keys()):
            unknown_variables = frozenset(equation.keys()) - frozenset(self.variables.keys())
            raise RuntimeError("Unknown variable(s): {0}".format(unknown_variables))
        simplified_equation = {
            variable: coefficient for variable, coefficient in equation.items()
            if coefficient is not None and coefficient != 0
        }
        if len(simplified_equation) == 0:
            raise RuntimeError("Empty equation")
        return simplified_equation

    def _normalise_variables(self):
        n = 0
        variable_factors = {}
        for variable, (lower_bound, upper_bound) in self.variables.items():
            variable_factors[variable] = {}
            if lower_bound is None or lower_bound < 0:
                variable_factors[variable][n] = -1
                n += 1
            if upper_bound is None or upper_bound > 0:
                variable_factors[variable][n] = 1
                n += 1
        return n, variable_factors

    def _build_tableau(self, num_variables, num_constraints, variable_factors):
        tableau = Tableau(num_variables, num_constraints)

        for i, (equation, value) in enumerate(self.constraints):
            for variable, coefficient in equation.items():
                for j, factor in variable_factors[variable].items():
                    tableau[i, j] = factor * coefficient
            tableau[i, num_variables + num_constraints] = value

        for variable, coefficient in self.objective.items():
            for j, factor in variable_factors[variable].items():
                tableau[num_constraints, j] = factor * coefficient

        for i, coefficient in self._slacks.items():
            tableau[i, num_variables + i] = coefficient

        return tableau, variable_factors


class Tableau:
    def __init__(self, num_variables, num_constraints):
        self.num_variables = num_variables
        self.num_constraints = num_constraints
        self.width = num_variables + num_constraints + 1
        self.height = num_constraints + 1

        initializer = (0 for _ in range(self.width * self.height))
        self._data = array('d', initializer)

    def __getitem__(self, coordinates):
        offset = self._get_offset(coordinates)
        return self._data[offset]

    def __setitem__(self, coordinates, value):
        offset = self._get_offset(coordinates)
        self._data[offset] = value

    def get_row_data(self, i):
        row_start = self._get_offset((i, 0))
        row_end = row_start + self.width
        row_data = self._data[row_start:row_end]
        return row_data

    def get_col_data(self, j):
        col_start = self._get_offset((0, j))
        col_end = col_start + self.width * self.num_constraints
        col_data = self._data[col_start:col_end:self.width]
        return col_data

    def find_pivot_column(self):
        row_data = self.get_row_data(self.height - 1)[:-1]
        max_j, max_value = None, None
        for j, value in enumerate(row_data):
            if value > 0 and (max_j is None or value > max_value):
                max_j = j
                max_value = value
        return max_j

    def find_pivot_row(self, pivot_column):
        min_i, min_ratio = None, None
        for i in range(self.num_constraints):
            value = self[i, pivot_column]
            if value == 0:
                continue
            b = self[i, self.width - 1]
            if b <= 0:
                continue
            ratio = b / value
            if min_i is None or ratio < min_ratio:
                min_i = i
                min_ratio = ratio
        return min_i

    def pivot(self, pivot_i, pivot_j):
        self.divide_row(pivot_i, self[pivot_i, pivot_j])
        for k in range(self.height):
            if k == pivot_i:
                continue
            factor = self[k, pivot_j]
            for l in range(self.width):
                self[k, l] = self[k, l] - factor * self[pivot_i, l]

    def divide_row(self, i, factor):
        row_start = self._get_offset((i, 0))
        row_end = row_start + self.width
        for offset in range(row_start, row_end):
            self._data[offset] /= factor

    def is_optimal(self):
        row_data = self.get_row_data(self.height - 1)
        return all(value <= 0 for value in row_data)

    def primal_solution(self):
        return [
            (j, self.variable_value_for_pivot_col(j))
            for j in range(self.width-1)
            if self.is_pivot_col(j)
        ]

    def get_objective_value(self):
        return -self[self.height-1, self.width-1]

    def is_pivot_col(self, j):
        col_data = self.get_col_data(j)
        return all(c == 0 or c == 1 for c in col_data) and sum(col_data) == 1

    def variable_value_for_pivot_col(self, j):
        col_data = self.get_col_data(j)
        pivot_i = col_data.index(1)
        return self[pivot_i, self.width-1]

    def _get_offset(self, coordinates):
        i, j = coordinates
        return self.width * i + j

    def echo(self):
        for line in self.lines():
            print(line)
        print()

    def lines(self):
        for i in range(self.height):
            yield " ".join("{0:< 8.3}".format(self[i, j]) for j in range(self.width))
