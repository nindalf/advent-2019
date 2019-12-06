use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|s| {
            let mut objects = s.trim().split(')');
            let left = hash(objects.next().unwrap());
            let right = hash(objects.next().unwrap());
            (left, right)
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn number_of_orbits(orbits: &[(u32, u32)]) -> u32 {
    steps_from_origin(orbits, hash("COM")).values().sum()
}

#[aoc(day6, part2)]
pub fn steps_to_santa(orbits: &[(u32, u32)]) -> u32 {
    steps_from_origin(orbits, hash("YOU"))
        .get(&hash("SAN"))
        .unwrap()
        - 2
}

#[derive(Debug)]
struct CelestialBody {
    satellites: HashSet<u32>,
    parent: Option<u32>,
}

fn hash(input: &str) -> u32 {
    let mut hash = 0;
    for (i, c) in input.chars().enumerate() {
        hash += (c as u32) * 100u32.pow(i as u32);
    }
    hash
}

fn get_graph(orbits: &[(u32, u32)]) -> HashMap<u32, CelestialBody> {
    let mut celestial_objs: HashMap<u32, CelestialBody> = HashMap::new();
    for (left, right) in orbits {
        let left_body = celestial_objs.entry(*left).or_insert(CelestialBody {
            satellites: HashSet::<u32>::new(),
            parent: None,
        });
        left_body.satellites.insert(*right);

        let right_body = celestial_objs.entry(*right).or_insert(CelestialBody {
            satellites: HashSet::<u32>::new(),
            parent: None,
        });
        right_body.parent = Some(*left);
    }
    celestial_objs
}

pub fn steps_from_origin(orbits: &[(u32, u32)], origin: u32) -> HashMap<u32, u32> {
    let celestial_objs = get_graph(orbits);
    let mut steps = 0;
    let mut current_gen = VecDeque::new();
    let mut next_gen = VecDeque::new();
    let mut visited = HashMap::new();
    current_gen.push_back(origin);

    while !current_gen.is_empty() || !next_gen.is_empty() {
        if current_gen.is_empty() {
            current_gen = next_gen;
            next_gen = VecDeque::new();
            steps += 1;
        }
        let next = current_gen.pop_front().unwrap();
        visited.insert(next, steps);
        let next_obj = celestial_objs.get(&next).unwrap();
        for satellite in &next_obj.satellites {
            if !visited.contains_key(satellite) {
                next_gen.push_back(*satellite);
            }
        }
        if next_obj.parent.is_some() && !visited.contains_key(&next_obj.parent.unwrap()) {
            next_gen.push_back(next_obj.parent.unwrap());
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_orbit() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
        let input = super::input_generator(&input);
        assert_eq!(super::number_of_orbits(&input), 42);
    }

    #[test]
    fn test_steps_to_santa() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
        let input = super::input_generator(&input);
        assert_eq!(super::steps_to_santa(&input), 4);
    }
}
