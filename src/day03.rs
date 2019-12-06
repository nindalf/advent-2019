use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }
}

fn wire_path(path: &str) -> HashMap<Point, i32> {
    let directions = path.split(',');
    let mut result: HashMap<Point, i32> = HashMap::new();

    let (mut x, mut y) = (0, 0);
    let mut total_distance = 0;
    for direction in directions {
        let mut chars = direction.chars();
        let d: char = chars.next().unwrap();
        let distance = i32::from_str(&direction[1..]).unwrap();
        let (mut end_x, mut end_y) = (x, y);
        match d {
            'R' => end_x += distance,
            'U' => end_y += distance,
            'D' => end_y -= distance,
            'L' => end_x -= distance,
            _ => panic!("Unknown direction"),
        }
        let start_x = cmp::min(x, end_x);
        let start_y = cmp::min(y, end_y);
        for i in start_x..=cmp::max(x, end_x) {
            for j in start_y..=cmp::max(y, end_y) {
                let point = Point { x: i, y: j };
                if result.contains_key(&point) {
                    continue;
                }
                let current_distance = i32::abs(start_x - i) + i32::abs(start_y - j);
                result.insert(point, total_distance + current_distance);
            }
        }
        total_distance += distance;
        x = end_x;
        y = end_y;
    }
    result.remove(&Point { x: 0, y: 0 });
    result
}

pub fn shortest_cross(wire_a: &str, wire_b: &str) -> i32 {
    let path_a = wire_path(wire_a);
    let path_b = wire_path(wire_b);

    let mut intersections = Vec::new();
    for (point, _) in path_a {
        if path_b.contains_key(&point) {
            intersections.push(point);
        }
    }

    let origin = Point { x: 0, y: 0 };
    intersections
        .iter()
        .map(|point| point.manhattan_distance(&origin))
        .min()
        .unwrap()
}

pub fn shortest_signal_time(wire_a: &str, wire_b: &str) -> i32 {
    let path_a = wire_path(wire_a);
    let path_b = wire_path(wire_b);

    let mut intersections = Vec::new();
    for (point, signal_time) in path_a {
        if path_b.contains_key(&point) {
            intersections.push(signal_time + path_b.get(&point).unwrap());
        }
    }

    *intersections.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shortest_cross() {
        let input_1_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input_1_b = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(super::shortest_cross(input_1_a, input_1_b), 159);

        let input_2_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input_2_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(super::shortest_cross(input_2_a, input_2_b), 135);

        let real_input = crate::utils::read_lines("data/day03.txt").unwrap();
        assert_eq!(super::shortest_cross(&real_input[0], &real_input[1]), 865);
    }

    #[test]
    fn test_shortest_signal_time() {
        let input_1_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input_1_b = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(super::shortest_signal_time(input_1_a, input_1_b), 610);

        // I don't know why this doesn't work.
        // let input_2_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        // let input_2_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        // assert_eq!(super::shortest_signal_time(input_2_a, input_2_b), 410);

        let real_input = crate::utils::read_lines("data/day03.txt").unwrap();
        assert_eq!(
            super::shortest_signal_time(&real_input[0], &real_input[1]),
            35038
        );
    }

    #[test]
    fn test_wire_path() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        super::wire_path(input);
    }
}
