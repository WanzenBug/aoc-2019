use std::{
    error::Error,
};
use std::fmt::{Display, Formatter};

const INPUT: &'static [u8] = include_bytes!("../INPUT");

struct Layer<'a> {
    pixels: &'a [u8],
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

impl<'a> Layer<'a> {
    fn new(data: &'a [u8]) -> Self {
        Layer { pixels: data }
    }
}

#[derive(Copy, Clone, Debug)]
enum PixelValue {
    WHITE,
    BLACK,
    TRANSPARENT,
}

impl Display for PixelValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PixelValue::*;
        let c = match self {
            &WHITE => " ",
            &BLACK => "#",
            &TRANSPARENT => "/",
        };
        write!(f, "{}", c)
    }
}

impl From<u8> for PixelValue {
    fn from(d: u8) -> Self {
        match d {
            b'0' => PixelValue::WHITE,
            b'1' => PixelValue::BLACK,
            b'2' => PixelValue::TRANSPARENT,
            x => panic!("Unknown conversion from {} to PixelValue", x)
        }
    }
}

impl PixelValue {
    fn add_backing_pixel(&mut self, other: PixelValue) {
        use PixelValue::*;
        let new = match &self {
            &TRANSPARENT => other,
            x => **x,
        };

        *self = new;
    }
}

struct Image {
    pixels: [PixelValue; WIDTH * HEIGHT],
}

impl Image {
    fn from_layers(layers: Vec<Layer>) -> Self {
        let mut image = Image { pixels: [PixelValue::TRANSPARENT; WIDTH * HEIGHT] };
        for layer in layers {
            for (cur, backing) in Iterator::zip(image.pixels.iter_mut(), layer.pixels.iter()) {
                cur.add_backing_pixel((*backing).into())
            }
        }

        image
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                write!(f, "{}", self.pixels[i * WIDTH + j])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT)?;
    println!("{}", result);
    Ok(())
}

fn run(input: &[u8]) -> Result<Image, Box<dyn Error + 'static>> {
    let layer_data_size = WIDTH * HEIGHT;
    let layers: Vec<_> = input
        .chunks_exact(layer_data_size)
        .map(Layer::new)
        .collect();

    Ok(Image::from_layers(layers))
}

#[cfg(test)]
mod tests {
    use super::*;
}

