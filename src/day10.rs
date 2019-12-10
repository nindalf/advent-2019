use std::collections::BTreeMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            let line = line.trim();
            let mut output = Vec::with_capacity(line.len());
            
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                if c == '.' {
                    output.push(0);
                }
                if c == '#' {
                    output.push(-1)
                }
            }
            
            output
        }).collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[aoc(day10, part1)]
pub fn most_asteroids_seen(input: &Vec<Vec<i32>>) -> i32 {
    best_asteroid(&input).1
}

#[aoc(day10, part2)]
pub fn find_200th_destroyed_asteroid(input: &Vec<Vec<i32>>) -> i32 {
    find_kth_destroyed_asteroid(&input, 200)
}

fn best_asteroid(input:  &Vec<Vec<i32>>) -> (Point, i32) {
    let mut best_point = Point{x:0, y:0};
    let mut max_so_far = find_visible_asteroids(&input, 0, 0);
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == -1 {
                let visible_asteroids = find_visible_asteroids(&input, x, y);
                if visible_asteroids > max_so_far {
                    max_so_far = visible_asteroids;
                    best_point = Point{x: x as i32, y: y as i32};
                }
            }
        }
    }

    (best_point, max_so_far)
}

fn find_visible_asteroids(input: &Vec<Vec<i32>>, start_x: usize, start_y: usize) -> i32 {
    let mut result = 0;
    for other_y in 0..input.len() {
        for other_x in 0..input[0].len() {
            // println!("{} {} {} {}", start_x, start_y, other_x, other_x);
            if input[other_y][other_x] == -1 && is_visible(&input, start_x as i32, start_y as i32, other_x as i32, other_y as i32) {
                result += 1;
            }
        }
    }

    result
}

use num::integer::Integer;

fn is_visible(input: &Vec<Vec<i32>>, start_x: i32, start_y: i32, other_x: i32, other_y: i32) -> bool {
    if start_x == other_x && start_y == other_y {
        return false;
    }
    let gcd = (other_x - start_x).gcd(&(other_y - start_y));
    let step_x = (other_x - start_x)/gcd;
    let step_y = (other_y - start_y)/gcd;
    let mut x = step_x + start_x;
    let mut y = step_y + start_y;
    while x >= 0 && x < input[0].len() as i32 && y >= 0 && y < input.len() as i32 {
        if input[y as usize][x as usize] == -1 {
            return y == other_y && x == other_x;
        }
        x += step_x;
        y += step_y;
    }
    false
}

fn find_kth_destroyed_asteroid(_input: &Vec<Vec<i32>>, _k: i32) -> i32 {
    // let best_asteroid = best_asteroid(&input).0;
    // let right_asteroids: BTreeMap<f64, Vec<Point>> = BTreeMap::new();
    // let left_asteroids: BTreeMap<f64, Vec<Point>> = BTreeMap::new();
    // for y in 0..input.len() {
    //     for x in 0..input[0].len() {
    //         if input[y][x] == -1 {
    //             let (y_diff, x_diff) = ((y - best_asteroid.y) as f64, (x - best_asteroid.x) as f64);
    //             let slope = if x_diff != 0 {
    //                 y_diff/x_diff
    //             } else {
    //                 if y_diff > 0 {std::f64::INFINITY} else {std::f64::NEG_INFINITY}
    //             };
    //             if x_diff > 0 {
    //                 right_asteroids.entry(slope).or_insert(Vec::new()).push(Point{x: x as i32, y: y as i32});
    //             } else {
    //                 left_asteroids.entry(slope).or_insert(Vec::new()).push(Point{x: x as i32, y: y as i32});
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", right_asteroids);
    // println!("{:?}", left_asteroids);
    // for value in right_asteroids.values() {
    //     return value.x * 100 + value.y;
    // }
    0
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_helper() {}

    #[test]
    fn test_visible_asteroids() {
        let input = super::input_generator(".#..#
        .....
        #####
        ....#
        ...##");
        assert_eq!(super::find_visible_asteroids(&input, 2, 2), 7);
        assert_eq!(super::find_visible_asteroids(&input, 0, 2), 6);
        assert_eq!(super::find_visible_asteroids(&input, 4, 2), 5);
        assert_eq!(super::find_visible_asteroids(&input, 3, 4), 8);
    }

    #[test]
    fn test_most_asteroids_seen() {
        let input = super::input_generator(".#..#
        .....
        #####
        ....#
        ...##");
        assert_eq!(super::most_asteroids_seen(&input), 8);

        let input = super::input_generator("......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####");
        assert_eq!(super::most_asteroids_seen(&input), 33);

        let input = super::input_generator("#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.");
        assert_eq!(super::most_asteroids_seen(&input), 35);

        
        let input = super::input_generator(".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..");
        assert_eq!(super::most_asteroids_seen(&input), 41);

        
        let input = super::input_generator(".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##");
        assert_eq!(super::most_asteroids_seen(&input), 210);
    }

    #[test]
    fn test_find_kth_destroyed_asteroid() {
        let input = super::input_generator(".#....#####...#..
        ##...##.#####..##
        ##...#...#.#####.
        ..#.....X...###..
        ..#.#.....#....##");
        // TODO implement and fix
        assert_eq!(super::find_kth_destroyed_asteroid(&input, 1), 1112);
    }

    // #[test]
    // fn test_part2() {}
}
