use std::{
    error::Error,
};

const INPUT: &'static [u8] = include_bytes!("../INPUT");

struct Layer<'a> {
    pixels: &'a [u8],
}

impl<'a> Layer<'a> {
    fn new(data: &'a [u8]) -> Self {
        Layer { pixels: data }
    }

    fn width() -> usize {
        25
    }

    fn height() -> usize {
        6
    }

    fn count_pixels(&self, d: u8) -> usize {
        self.pixels
            .iter()
            .filter(|&&v| v == d)
            .count()
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT)?;
    eprintln!("result = {:#?}", result);

    Ok(())
}

fn run(input: &[u8]) -> Result<usize, Box<dyn Error + 'static>> {
    let layer_data_size = Layer::width() * Layer::height();
    let mut layers: Vec<_> = input
        .chunks_exact(layer_data_size)
        .map(Layer::new)
        .collect();

    layers.sort_by_key(|l|l.count_pixels(b'0'));
    let min_zero_layer = &layers[0];
    Ok(min_zero_layer.count_pixels(b'1') * min_zero_layer.count_pixels(b'2'))
}

#[cfg(test)]
mod tests {
    use super::*;

}

