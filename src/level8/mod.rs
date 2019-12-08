use std::fs;
use std::error::Error;
use std::fmt;

struct Layer {
    pixels: Vec<u32>,
    width: u32,
    height: u32
}

impl Layer {
    pub fn new(pixels: Vec<u32>, width: u32, height: u32) -> Self {
        assert!(pixels.len() as u32 == width * height);
        Layer {
            pixels,
            width,
            height
        }
    }
    fn count(&self, target:u32) -> u32 {
        self.pixels
            .iter()
            .filter(|&p| *p == target)
            .count() as u32
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.pixels.chunks(self.width as usize) {
            write!(f, "{:?}\n", chunk);
        }
        Ok(())
    }
}

// 0 = black, 1 = white , 2 = transparent
fn part2(layers: Vec<Layer>) -> Layer {
    let pixel_count = layers[0].pixels.len();
    let mut new_pixels = vec![];
    for i in 0..pixel_count {
        for layer in layers.iter() {
            if layer.pixels[i] != 2 {
                new_pixels.push(layer.pixels[i]);
                break;
            }
        }
    }
    Layer {
        pixels:new_pixels,
        width: layers[0].width,
        height: layers[0].height
    }
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/level8/input.txt")?;
    let digits: Vec<u32> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let layer_width: u32 = 25;
    let layer_height: u32 = 6;
    let layer_size = (layer_width * layer_height) as usize;
    let mut layers: Vec<Layer> = vec![];
    for chunk in digits.chunks(layer_size) {
        let chunk: Vec<u32> = chunk.to_vec();
        let layer = Layer::new(chunk, layer_width, layer_height);
        layers.push(layer);
    }
    let layer_zero_cnt: Vec<u32> = layers.iter().map(|l| l.count(0)).collect();
    let mut min_idx = 0;
    let mut min_layer_zero = layer_zero_cnt[0];
    for (i, &layer_zero) in layer_zero_cnt.iter().enumerate() {
        if layer_zero < min_layer_zero {
            min_idx = i;
            min_layer_zero = layer_zero;
        }
    }
    let target_layer : &Layer = layers.get(min_idx).unwrap();
    // println!("{}", target_layer.count(1) * target_layer.count(2));
    let fusion_layer = part2(layers);
    println!("{}", fusion_layer);
    Ok(())
}
