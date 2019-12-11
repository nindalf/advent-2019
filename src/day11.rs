use crate::computer::Computer;
use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

// 2469
#[aoc(day11, part1)]
pub fn area_covered(input: &[i64]) -> usize {
    paint_grid(input, 0, 110).1
}

// KLCZAEGU
#[aoc(day11, part2)]
pub fn print_registration(input: &[i64]) -> String {
    let grid = paint_grid(input, 1, 90).0;

    let mut result = vec!["\n"];
    for y in 45..51 {
        for x in 40..90 {
            if grid[y][x] == 1 {
                result.push("#");
            } else {
                result.push(" ");
            }
        }
        result.push("\n");
    }
    result.join("")
}

pub fn paint_grid(input: &[i64], initial_paint: i64, grid_size: usize) -> (Vec<Vec<i64>>, usize) {
    let mut grid = vec![vec![0; grid_size]; grid_size];
    let mut robot = Robot {
        x: grid_size / 2,
        y: grid_size / 2,
        direction: Direction::UP,
        computer: Computer::new(input),
    };
    grid[robot.y][robot.x] = initial_paint;

    let mut visited = HashSet::new();
    let (mut prev_x, mut prev_y) = (robot.x, robot.y);
    while let Some(paint) = robot.paint(grid[prev_y][prev_x]) {
        visited.insert((prev_x, prev_y));
        grid[prev_y][prev_x] = paint;
        prev_x = robot.x;
        prev_y = robot.y;
    }
    (grid, visited.len())
}

pub struct Robot {
    x: usize,
    y: usize,
    direction: Direction,
    computer: Computer,
}

impl Robot {
    fn paint(&mut self, current_paint: i64) -> Option<i64> {
        let new_paint = self.computer.compute(&vec![current_paint])?;
        let turn = self.computer.compute(&vec![])?;
        // can also be implemented as fromPrimitive(self.direction.as_u8() + 1)
        //  but I think the current code is more idiomatic
        self.direction = match turn {
            0 => match self.direction {
                Direction::UP => Direction::LEFT,
                Direction::LEFT => Direction::DOWN,
                Direction::DOWN => Direction::RIGHT,
                Direction::RIGHT => Direction::UP,
            },
            1 => match self.direction {
                Direction::UP => Direction::RIGHT,
                Direction::LEFT => Direction::UP,
                Direction::DOWN => Direction::LEFT,
                Direction::RIGHT => Direction::DOWN,
            },
            _ => panic!("Impossible direction"),
        };
        self.move_forward();
        Some(new_paint)
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::UP => self.y -= 1,
            Direction::LEFT => self.x -= 1,
            Direction::DOWN => self.y += 1,
            Direction::RIGHT => self.x += 1,
        }
    }
}

#[derive(Debug)]
enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}
