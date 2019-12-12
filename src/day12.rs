use num::integer::Integer;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Planet> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("<x=(?P<x>-?[0-9]*), y=(?P<y>-?[0-9]*), z=(?P<z>-?[0-9]*)>").unwrap();
    }
    input
        .lines()
        .map(|s| {
            let line = s.trim();
            let caps = RE.captures(line).unwrap();
            let x = i32::from_str(&caps["x"]).unwrap();
            let y = i32::from_str(&caps["y"]).unwrap();
            let z = i32::from_str(&caps["z"]).unwrap();
            let (v_x, v_y, v_z) = (0, 0, 0);
            Planet {
                x,
                y,
                z,
                v_x,
                v_y,
                v_z,
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn total_energy(input: &[Planet]) -> i32 {
    total_energy_after_k_steps(input, 1000)
}

#[aoc(day12, part2)]
pub fn steps_to_repeat(input: &[Planet]) -> i128 {
    let mut cycles = HashMap::new();

    let mut input_copy = input.to_vec();
    for k in 1.. {
        for i in 0..input.len() {
            for j in 0..input.len() {
                // TODO fix this nonsense
                let other = input_copy[j].clone();
                input_copy[i].update_velocity(&other);
            }
        }

        for i in 0..input.len() {
            input_copy[i].update_position();
        }

        for i in 0..input.len() {
            let trip_x = (i, 'x', input_copy[i].x, input_copy[i].v_x);
            let trip_y = (i, 'y', input_copy[i].y, input_copy[i].v_y);
            let trip_z = (i, 'z', input_copy[i].z, input_copy[i].v_z);

            if trip_x == (i, 'x', input[i].x, input[i].v_x) && !cycles.contains_key(&trip_x) {
                cycles.insert(trip_x, k);
                println!("len - {} {:?}", cycles.len(), cycles);
            }
            if trip_y == (i, 'y', input[i].y, input[i].v_y) && !cycles.contains_key(&trip_y) {
                cycles.insert(trip_y, k);
                println!("len - {} {:?}", cycles.len(), cycles);
            }
            if trip_z == (i, 'z', input[i].z, input[i].v_z) && !cycles.contains_key(&trip_z) {
                cycles.insert(trip_z, k);
                println!("len - {} {:?}", cycles.len(), cycles);
            }
        }
        if cycles.len() == 12 {
            break;
        }
    }
    let mut result = 1;
    for cycle in cycles.values() {
        result = result.lcm(cycle);
    }
    result
}

fn total_energy_after_k_steps(input: &[Planet], k: i32) -> i32 {
    let mut input = input.to_vec();
    for _ in 0..k {
        for i in 0..input.len() {
            for j in 0..input.len() {
                // TODO fix this nonsense
                let other = input[j].clone();
                input[i].update_velocity(&other);
            }
        }
        for i in 0..input.len() {
            input[i].update_position();
        }
    }
    input.iter().map(|planet| planet.total_energy()).sum()
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Planet {
    x: i32,
    y: i32,
    z: i32,
    v_x: i32,
    v_y: i32,
    v_z: i32,
}

impl Planet {
    fn total_energy(&self) -> i32 {
        let position = i32::abs(self.x) + i32::abs(self.y) + i32::abs(self.z);
        let velocity = i32::abs(self.v_x) + i32::abs(self.v_y) + i32::abs(self.v_z);
        position * velocity
    }

    fn update_position(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
        self.z += self.v_z;
    }

    fn update_velocity(&mut self, other: &Planet) {
        self.v_x += if self.x > other.x {
            -1
        } else if self.x < other.x {
            1
        } else {
            0
        };

        self.v_y += if self.y > other.y {
            -1
        } else if self.y < other.y {
            1
        } else {
            0
        };

        self.v_z += if self.z > other.z {
            -1
        } else if self.z < other.z {
            1
        } else {
            0
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_total_energy() {
        let input = super::input_generator(
            "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>",
        );
        assert_eq!(super::total_energy_after_k_steps(&input, 10), 179);

        let input = super::input_generator(
            "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>",
        );
        assert_eq!(super::total_energy_after_k_steps(&input, 100), 1940);
    }

    #[test]
    fn test_steps_to_repeat() {
        let input = super::input_generator(
            "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>",
        );
        assert_eq!(super::steps_to_repeat(&input), 2772);

        let input = super::input_generator(
            "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>",
        );
        assert_eq!(super::steps_to_repeat(&input), 4686774924);
    }
}
