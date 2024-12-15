pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day14.txt");

    let mut lobby = Lobby::from_lines(data.clone(), Vector(101, 103));
    lobby.progress(100);
    let part1 = lobby.safety_factor();

    let mut lobby = Lobby::from_lines(data, Vector(101, 103));
    let start = 5000;
    let end = 10000;
    lobby.progress(start);
    let mut scores = vec![];
    for i in start..end {
        scores.push((i, lobby.score()));
        lobby.progress(1);
    }
    scores.sort_by_key(|&(i, score)| (score, -i));

    let part2 = scores.last().unwrap().0;

    format!("{} {}", part1, part2)
}

type Dimension = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vector(Dimension, Dimension);

impl Vector {
    fn from_text(s: &str) -> Vector {
        let parts = s.split_once(",").unwrap();
        let x = parts.0.parse::<Dimension>().unwrap();
        let y = parts.1.parse::<Dimension>().unwrap();

        Vector(x, y)
    }
}

#[derive(Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn from_text(s: &str) -> Robot {
        let (position_str, velocity_str) = s.split_once(" ").unwrap();

        assert!(position_str.starts_with("p="));
        assert!(velocity_str.starts_with("v="));

        let position = Vector::from_text(&position_str[2..]);
        let velocity = Vector::from_text(&velocity_str[2..]);

        Robot { position, velocity }
    }

    fn progress(&mut self, size: &Vector, seconds: Dimension) {
        self.position.0 += seconds * self.velocity.0;
        if self.position.0 < 0 {
            let loops = 1 - self.position.0 / size.0;
            self.position.0 += loops * size.0;
        }
        self.position.0 %= size.0;

        self.position.1 += seconds * self.velocity.1;
        if self.position.1 < 0 {
            let loops = 1 - self.position.1 / size.1;
            self.position.1 += loops * size.1;
        }
        self.position.1 %= size.1;
    }
}

struct Lobby {
    robots: Vec<Robot>,
    size: Vector,
}

impl Lobby {
    fn from_lines(lines: Vec<String>, size: Vector) -> Lobby {
        let robots = lines.iter().map(|line| Robot::from_text(line)).collect();
        assert_eq!(size.0 % 2, 1);
        assert_eq!(size.1 % 2, 1);
        Lobby { robots, size }
    }

    fn progress(&mut self, seconds: Dimension) {
        self.robots
            .iter_mut()
            .for_each(|robot| robot.progress(&self.size, seconds))
    }

    fn safety_factor(&self) -> usize {
        let quadrants = self.quadrant_counts();

        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    }

    fn quadrant_counts(&self) -> [usize; 5] {
        let mut counts = [0, 0, 0, 0, 0];
        let middle = Vector((self.size.0 - 1) / 2, (self.size.1 - 1) / 2);

        for robot in self.robots.iter() {
            let mut quadrant = 0;
            if robot.position.0 != middle.0 && robot.position.1 != middle.1 {
                if robot.position.0 > middle.0 {
                    quadrant += 1;
                }
                if robot.position.1 > middle.1 {
                    quadrant += 2;
                }
            } else {
                quadrant = 4;
            }
            counts[quadrant] += 1;
        }
        counts
    }

    fn print(&self) {
        let mut pixels = vec![vec![' '; self.size.0 as usize]; self.size.1 as usize];
        for robot in self.robots.iter() {
            pixels[robot.position.1 as usize][robot.position.0 as usize] = '#';
        }
        for row in pixels {
            println!("{}", row.into_iter().collect::<String>());
        }
    }

    fn score(&self) -> usize {
        let mut presence = vec![vec![false; self.size.0 as usize]; self.size.1 as usize];
        for robot in self.robots.iter() {
            presence[robot.position.1 as usize][robot.position.0 as usize] = true;
        }

        let mut score = 0;
        for row in presence {
            for i in 1..row.len() {
                if row[i] && row[i - 1] {
                    score += 1;
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "225521010 7774");
    }

    impl PartialEq<(Dimension, Dimension)> for Vector {
        fn eq(&self, other: &(Dimension, Dimension)) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    #[test]
    fn test_coord_from_text() {
        assert_eq!(Vector::from_text("0,0"), (0, 0));
        assert_eq!(Vector::from_text("19,25"), (19, 25));
        assert_eq!(Vector::from_text("-39,-15"), (-39, -15));
        assert_eq!(Vector::from_text("7,-35"), (7, -35));
        assert_eq!(Vector::from_text("-13,42"), (-13, 42));
    }

    #[test]
    fn test_robot_from_text() {
        let examples = _example();

        assert_eq!(Robot::from_text(&examples[0]).position, (0, 4));
        assert_eq!(Robot::from_text(&examples[1]).position, (6, 3));
        assert_eq!(Robot::from_text(&examples[2]).position, (10, 3));
        assert_eq!(Robot::from_text(&examples[11]).position, (9, 5));

        assert_eq!(Robot::from_text(&examples[0]).velocity, (3, -3));
        assert_eq!(Robot::from_text(&examples[1]).velocity, (-1, -3));
        assert_eq!(Robot::from_text(&examples[2]).velocity, (-1, 2));
        assert_eq!(Robot::from_text(&examples[11]).velocity, (-3, -3));
    }

    #[test]
    fn test_robot_progress() {
        let size = Vector(11, 7);
        let mut robot = Robot::from_text("p=2,4 v=2,-3");

        assert_eq!(robot.position, Vector(2, 4));
        robot.progress(&size, 1);
        assert_eq!(robot.position, Vector(4, 1));
        robot.progress(&size, 1);
        assert_eq!(robot.position, Vector(6, 5));
        robot.progress(&size, 2);
        assert_eq!(robot.position, Vector(10, 6));
    }

    #[test]
    fn test_lobby() {
        let size = Vector(11, 7);
        let mut example = Lobby::from_lines(_example(), size);

        assert_eq!(example.robots.len(), 12);
        assert_eq!(example.robots[6].position, (7, 6));
        assert_eq!(example.safety_factor(), 0);

        example.progress(100);
        assert_eq!(example.safety_factor(), 12);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("p=0,4 v=3,-3"),
            String::from("p=6,3 v=-1,-3"),
            String::from("p=10,3 v=-1,2"),
            String::from("p=2,0 v=2,-1"),
            String::from("p=0,0 v=1,3"),
            String::from("p=3,0 v=-2,-2"),
            String::from("p=7,6 v=-1,-3"),
            String::from("p=3,0 v=-1,-2"),
            String::from("p=9,3 v=2,3"),
            String::from("p=7,3 v=-1,2"),
            String::from("p=2,4 v=2,-3"),
            String::from("p=9,5 v=-3,-3"),
        ]
    }
}
