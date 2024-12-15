pub fn execute() -> String {
    let storm = HailStorm::from_lines(aoc_utils::read_lines("input/day24.txt"));
    let intersections = storm.valid_intersects_xy(200000000000000.0, 400000000000000.0);

    let part1 = intersections.len();

    let throw_position = storm.find_throw_position();
    let part2 = throw_position.x + throw_position.y + throw_position.z;

    format!("{} {}", part1, part2)
}

type Coordinate = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Vector {
    fn from_string(s: &str) -> Self {
        let (x_str, rest) = s.split_once(',').unwrap();
        let (y_str, z_str) = rest.split_once(',').unwrap();
        let x = x_str.trim().parse().unwrap();
        let y = y_str.trim().parse().unwrap();
        let z = z_str.trim().parse().unwrap();
        Vector { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Stone {
    position: Vector,
    velocity: Vector,
}

impl Stone {
    fn from_line(line: String) -> Stone {
        let (position_str, velocity_str) = line.split_once('@').unwrap();
        let position = Vector::from_string(position_str.trim());
        let velocity = Vector::from_string(velocity_str.trim());
        Stone { position, velocity }
    }

    fn intersects_xy(&self, other: &Stone) -> Option<Vector> {
        let den = self.velocity.x * other.velocity.y - self.velocity.y * other.velocity.x;

        if den == 0.0 {
            return None;
        }

        let num1 = other.velocity.x * (self.position.y - other.position.y)
            + other.velocity.y * (other.position.x - self.position.x);
        let num2 = self.velocity.x * (self.position.y - other.position.y)
            + self.velocity.y * (other.position.x - self.position.x);

        if num1 * den < 0.0 || num2 * den < 0.0 {
            return None;
        }

        let x = self.position.x + self.velocity.x * num1 / den;
        let y = self.position.y + self.velocity.y * num1 / den;

        let z = 0.0;

        Some(Vector { x, y, z })
    }
}

struct HailStorm {
    stones: Vec<Stone>,
}
impl HailStorm {
    fn from_lines(lines: Vec<String>) -> HailStorm {
        let stones = lines.into_iter().map(|l| Stone::from_line(l)).collect();
        HailStorm { stones }
    }

    fn valid_intersects_xy(&self, min: Coordinate, max: Coordinate) -> Vec<Vector> {
        let mut result = Vec::new();
        for (i, a) in self.stones.iter().enumerate() {
            for b in self.stones[i + 1..].iter() {
                let intersect = a.intersects_xy(b);
                if intersect.is_some_and(|intersect| {
                    intersect.x >= min
                        && intersect.x <= max
                        && intersect.y >= min
                        && intersect.y <= max
                }) {
                    result.push(intersect.unwrap());
                }
            }
        }
        result
    }

    fn find_throw_position(&self) -> Vector {
        use z3::ast::Ast;
        use z3::*;

        let config = Config::new();
        let context = Context::new(&config);
        let solver = Solver::new(&context);

        // Unknowns are:
        // - position to throw from (x0, y0, z0)
        // - velocity to throw with (u0, v0, w0)
        // - time of colisions with stones t1, t2, t3
        let x_0 = ast::Int::new_const(&context, "x0");
        let y_0 = ast::Int::new_const(&context, "y0");
        let z_0 = ast::Int::new_const(&context, "z0");
        let u_0 = ast::Int::new_const(&context, "u0");
        let v_0 = ast::Int::new_const(&context, "v0");
        let w_0 = ast::Int::new_const(&context, "w0");

        let zero = ast::Int::from_i64(&context, 0);

        for i in 0..self.stones.len() {
            let t_n = ast::Int::new_const(&context, format!("t{}", i + 1));

            let stone = &self.stones[i];

            let x_n = ast::Int::from_i64(&context, stone.position.x as i64);
            let y_n = ast::Int::from_i64(&context, stone.position.y as i64);
            let z_n = ast::Int::from_i64(&context, stone.position.z as i64);
            let u_n = ast::Int::from_i64(&context, stone.velocity.x as i64);
            let v_n = ast::Int::from_i64(&context, stone.velocity.y as i64);
            let w_n = ast::Int::from_i64(&context, stone.velocity.z as i64);

            let eq_x = &x_0 + &u_0 * &t_n - x_n - u_n * &t_n;
            let eq_y = &y_0 + &v_0 * &t_n - y_n - v_n * &t_n;
            let eq_z = &z_0 + &w_0 * &t_n - z_n - w_n * &t_n;

            solver.assert(&eq_x._eq(&zero));
            solver.assert(&eq_y._eq(&zero));
            solver.assert(&eq_z._eq(&zero));
        }

        if matches!(solver.check(), SatResult::Sat) {
            if let Some(model) = solver.get_model() {
                return Vector {
                    x: model.get_const_interp(&x_0).unwrap().as_i64().unwrap() as Coordinate,
                    y: model.get_const_interp(&y_0).unwrap().as_i64().unwrap() as Coordinate,
                    z: model.get_const_interp(&z_0).unwrap().as_i64().unwrap() as Coordinate,
                };
            }
        }

        panic!("No solution found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "15889 801386475216902");
    }

    #[test]
    fn test_stone_from_line() {
        let ex1 = Stone::from_line("9, 13, 0 @ -2,  1, -2".to_string());
        assert_eq!(
            Vector {
                x: 9.0,
                y: 13.0,
                z: 0.0
            },
            ex1.position
        );
        assert_eq!(
            Vector {
                x: -2.0,
                y: 1.0,
                z: -2.0
            },
            ex1.velocity
        );
    }

    #[test]
    fn test_intersect_xy() {
        let example = HailStorm::from_lines(_example());

        fn check_xy(ex: Coordinate, ey: Coordinate, intersect: Vector) {
            assert!(
                Coordinate::abs(intersect.x - ex) < 0.000000001,
                "X mismatch: {} vs {}",
                ex,
                intersect.x
            );
            assert!(
                Coordinate::abs(intersect.y - ey) < 0.000000001,
                "Y mismatch: {} vs {}",
                ey,
                intersect.y
            );
        }

        let intersect_0_1 = example.stones[0].intersects_xy(&example.stones[1]);
        check_xy(14.0 + 1.0 / 3.0, 15.0 + 1.0 / 3.0, intersect_0_1.unwrap());
        let intersect_0_2 = example.stones[0].intersects_xy(&example.stones[2]);
        check_xy(11.0 + 2.0 / 3.0, 16.0 + 2.0 / 3.0, intersect_0_2.unwrap());
        let intersect_0_3 = example.stones[0].intersects_xy(&example.stones[3]);
        check_xy(6.2, 19.4, intersect_0_3.unwrap());
        let intersect_0_4 = example.stones[0].intersects_xy(&example.stones[4]);
        assert_eq!(intersect_0_4, None);

        let intersect_1_2 = example.stones[1].intersects_xy(&example.stones[2]);
        assert_eq!(intersect_1_2, None);
        let intersect_1_3 = example.stones[1].intersects_xy(&example.stones[3]);
        check_xy(-6.0, -5.0, intersect_1_3.unwrap());
        let intersect_1_4 = example.stones[1].intersects_xy(&example.stones[4]);
        assert_eq!(intersect_1_4, None);

        let intersect_2_3 = example.stones[2].intersects_xy(&example.stones[3]);
        check_xy(-2.0, 3.0, intersect_2_3.unwrap());
        let intersect_2_4 = example.stones[2].intersects_xy(&example.stones[4]);
        assert_eq!(intersect_2_4, None);

        let intersect_3_4 = example.stones[3].intersects_xy(&example.stones[4]);
        assert_eq!(intersect_3_4, None);
    }

    #[test]
    fn test_hailstorm_from_line() {
        let lines = _example();
        let hailstorm = HailStorm::from_lines(lines);

        assert_eq!(5, hailstorm.stones.len());

        assert_eq!(
            Vector {
                x: 19.0,
                y: 13.0,
                z: 30.0,
            },
            hailstorm.stones[0].position
        );
        assert_eq!(
            Vector {
                x: -2.0,
                y: 1.0,
                z: -2.0
            },
            hailstorm.stones[0].velocity
        );

        assert_eq!(
            Vector {
                x: 20.0,
                y: 19.0,
                z: 15.0,
            },
            hailstorm.stones[4].position
        );
        assert_eq!(
            Vector {
                x: 1.0,
                y: -5.0,
                z: -3.0
            },
            hailstorm.stones[4].velocity
        );
    }

    #[test]
    fn test_valid_intersects_xy() {
        let example = HailStorm::from_lines(_example());
        let intersections = example.valid_intersects_xy(7.0, 27.0);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_find_throw_position() {
        let example = HailStorm::from_lines(_example());
        let position = example.find_throw_position();
        assert_eq!(position.x, 24.0);
        assert_eq!(position.y, 13.0);
        assert_eq!(position.z, 10.0);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("19, 13, 30 @ -2,  1, -2"),
            String::from("18, 19, 22 @ -1, -1, -2"),
            String::from("20, 25, 34 @ -2, -2, -4"),
            String::from("12, 31, 28 @ -1, -2, -1"),
            String::from("20, 19, 15 @  1, -5, -3"),
        ]
    }
}
