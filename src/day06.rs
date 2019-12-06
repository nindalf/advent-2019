use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn get_graph(orbits: &[String]) -> HashMap<u32, CelestialBody> {
    let mut celestial_objs: HashMap<u32, CelestialBody> = HashMap::new();
    for orbit in orbits {
        
        let mut objects = orbit.split(')');
        let left = hash(objects.next().unwrap());
        let right = hash(objects.next().unwrap());
        let left_body = celestial_objs.entry(left).or_insert(CelestialBody {
            satellites: HashSet::<u32>::new(),
            parent: None,
        });
        left_body.satellites.insert(right);

        let right_body = celestial_objs.entry(right).or_insert(CelestialBody {
            satellites: HashSet::<u32>::new(),
            parent: None,
        });
        right_body.parent = Some(left);
    }
    celestial_objs
}

#[allow(dead_code)]
fn number_of_orbits(orbits: &[String]) -> u32 {
    steps_from_origin(orbits, hash("COM")).values().sum()
}

#[allow(dead_code)]
fn steps_to_santa(orbits: &[String]) -> u32 {
    steps_from_origin(orbits, hash("YOU")).get(&hash("SAN")).unwrap() - 2
}

pub fn steps_from_origin(orbits: &[String], origin: u32) -> HashMap<u32, u32> {
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
        let input: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(super::number_of_orbits(&input), 42);

        let input = crate::utils::read_lines("data/day06.txt").unwrap();
        assert_eq!(super::number_of_orbits(&input), 223251);
    }

    #[test]
    fn test_steps_to_santa() {
        let input: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(super::steps_to_santa(&input), 4);
        let input = crate::utils::read_lines("data/day06.txt").unwrap();
        assert_eq!(super::steps_to_santa(&input), 430);
    }

    fn convert(input: Vec<String>) -> Vec<(u32, u32)> {
        input.iter()
            .map(|s| {
                let mut objects = s.split(')');
                let left = super::hash(objects.next().unwrap());
                let right = super::hash(objects.next().unwrap());
                (left, right)
            }).collect()
    }
}
