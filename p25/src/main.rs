use std::{
    collections::HashMap,
    error::Error,
};

use crate::intcode::ProgramState;

mod intcode;


const INPUT: &'static str = include_str!("../INPUT");


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT)?;
    eprintln!("{}", result);

    Ok(())
}

fn run(input: &str) -> Result<usize, Box<dyn Error + 'static>> {
    let memory: Result<Vec<isize>, _> = input.split(',')
        .map(|part| part.trim().parse::<isize>())
        .collect();
    let memory = memory?;

    let mut screen: HashMap<(isize, isize), TileType> = HashMap::new();
    let mut prog = intcode::Program::new(memory);
    let mut all_out = Vec::new();
    loop {
        let (state, out) = prog.run(&mut None);
        all_out.extend_from_slice(&out);
        if let ProgramState::Halt = state {
            break;
        }
    }

    assert_eq!(all_out.len() % 3, 0);

    for output in all_out.chunks_exact(3) {
        let (x, y, tile) = (output[0], output[1], output[2].into());
        screen.insert((x, y), tile);
    }

    Ok(screen.values().map(|x| if let TileType::Block = x { 1 } else { 0 }).sum())
}

enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<isize> for TileType {
    fn from(v: isize) -> Self {
        use TileType::*;
        match v {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unimplemented!(),
        }
    }
}
