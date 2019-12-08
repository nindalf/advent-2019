use std::collections::HashMap;
use std::str::FromStr;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (String, String) {
    let lines: Vec<&str> = input.lines().map(|s| s.trim()).collect();
    (lines[0].to_owned(), lines[1].to_owned())
}

#[aoc(day3, part1)]
pub fn shortest_cross(wires: &(String, String)) -> i32 {
    let path_a = wire_path(&wires.0);
    let path_b = wire_path(&wires.1);

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

#[aoc(day3, part2)]
pub fn shortest_signal_time(wires: &(String, String)) -> i32 {
    let path_a = wire_path(&wires.0);
    let path_b = wire_path(&wires.1);

    let mut intersections = Vec::new();
    for (point, signal_time) in path_a {
        if path_b.contains_key(&point) {
            intersections.push(signal_time + path_b.get(&point).unwrap());
        }
    }

    *intersections.iter().min().unwrap()
}
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
    let mut result: HashMap<Point, i32> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    let mut total_distance = 0;
    
    let movements = path.split(',');
    for movement in movements {
        let distance = i32::from_str(&movement[1..]).unwrap();
        let (direction, end_x, end_y) = match movement.chars().next().unwrap() {
            'R' => (1, x+distance, y),
            'U' => (1, x, y+distance),
            'D' => (-1, x, y-distance),
            'L' => (-1, x-distance, y),
            _ => panic!("Unknown direction"),
        };

        for i in range(x, end_x, direction) {
            for j in range(y, end_y, direction) {
                let point = Point { x: i, y: j };
                if result.contains_key(&point) {
                    continue;
                }
                let current_distance = i32::abs(x - i) + i32::abs(y - j);
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

fn range(start:i32, end:i32, direction: i32) -> Box<dyn Iterator<Item=i32>> {
    if direction > 0 {
        return Box::new(start..=end)
    }
    Box::new((end..=start).rev())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shortest_cross() {
        let input = super::input_generator("R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(super::shortest_cross(&input), 159);

        let input = super::input_generator("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(super::shortest_cross(&input), 135);
    }

    #[test]
    fn test_shortest_signal_time() {
        let input = super::input_generator("R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(super::shortest_signal_time(&input), 610);

        let input = super::input_generator("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(super::shortest_signal_time(&input), 410);
    }
}
