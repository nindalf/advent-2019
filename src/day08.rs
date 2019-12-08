use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Image {
    let raw_data: Vec<u8> = input.trim().chars().map(|c| c as u8 - 48).collect();
    let height = 6;
    let width = 25;
    let layers = raw_data.len() / (height * width);
    Image {
        raw_data,
        height,
        width,
        layers,
    }
}

#[aoc(day8, part1)]
pub fn product_on_least_zeros(input: &Image) -> usize {
    let mut least_zeros = input.height * input.width;
    let mut result = 0;
    for z in 0..input.layers {
        let mut num_digits = HashMap::new();
        for y in 0..input.width {
            for x in 0..input.height {
                let pixel = input.get_pixel(x, y, z);
                *num_digits.entry(pixel).or_insert(0) += 1;
            }
        }
        if least_zeros > *num_digits.get(&0).unwrap_or(&0) {
            least_zeros = *num_digits.get(&0).unwrap_or(&0);
            result = *num_digits.get(&1).unwrap_or(&0) * *num_digits.get(&2).unwrap_or(&0);
        }
    }
    result
}

#[aoc(day8, part2)]
pub fn part2(input: &Image) -> i32 {
    let mut final_image = Image::new(input.height, input.width, 1);
    for x in 0..input.height {
        for y in 0..input.width {
            for z in 0..input.layers {
                let pixel = input.get_pixel(x, y, z);
                if pixel == 2 {
                    continue;
                }
                final_image.set_pixel(x, y, 0, pixel);
                break;
            }
        }
    }
    final_image.print_layer(0);

    0
}

pub struct Image {
    raw_data: Vec<u8>,
    height: usize,
    width: usize,
    layers: usize,
}

impl Image {
    pub fn get_pixel(&self, x: usize, y: usize, z: usize) -> u8 {
        self.raw_data[y + x * self.width + z * (self.height * self.width)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.raw_data[y + x * self.width + z * (self.height * self.width)] = val;
    }

    pub fn new(height: usize, width: usize, layers: usize) -> Image {
        let raw_data = vec![0; height * width * layers];
        Image {
            raw_data,
            height,
            width,
            layers,
        }
    }

    pub fn print_layer(&self, layer: usize) {
        for x in 0..self.height {
            for y in 0..self.width {
                if self.get_pixel(x, y, layer) == 1 {
                    print!("1");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
