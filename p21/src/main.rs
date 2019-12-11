use std::{
    collections::HashMap,
    error::Error,
};

mod intcode;


const INPUT: &'static str = include_str!("../INPUT");


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT)?;
    eprintln!("result = {:#?}", result);

    Ok(())
}

fn run(input: &str) -> Result<usize, Box<dyn Error + 'static>> {
    let memory: Result<Vec<isize>, _> = input.split(',')
        .map(|part| part.trim().parse::<isize>())
        .collect();
    let memory = memory?;


    let mut prog = intcode::Program::new(memory);
    let mut current_position = RobotPosition(0, 0);
    let mut current_direction = RobotDirection::Up;
    let mut hull_memory: HullMap = Default::default();

    loop {
        let cur_color = match hull_memory.get_color(current_position) {
            &Color::Black => 0,
            &Color::White => 1,
        };

        let (state, result) = prog.run(&mut Some(cur_color));
        if let intcode::ProgramState::Halt = state {
            break;
        }

        assert_eq!(result.len(), 2);
        let color = match result[0] {
            0 => Color::Black,
            1 => Color::White,
            _ => unimplemented!("Any color, as long as its black (or white)"),
        };
        hull_memory.paint(current_position, color);
        current_direction = match result[1] {
            0 => current_direction.rotate_left(),
            1 => current_direction.rotate_right(),
            _ => unimplemented!("Any direction as long as its left or right"),
        };

        current_position = current_position.move_forward(current_direction);
    }
    Ok(hull_memory.len())
}

#[derive(Debug, Default)]
struct HullMap(HashMap<RobotPosition, Color>);

#[derive(Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
enum RobotDirection {
    Up,
    Right,
    Down,
    Left,
}

impl HullMap {
    pub fn get_color(&self, pos: RobotPosition) -> &Color {
        self.0.get(&pos).unwrap_or(&Color::Black)
    }

    pub fn paint(&mut self, pos: RobotPosition, color: Color) {
        self.0.insert(pos, color);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl RobotDirection {
    pub fn rotate_left(self) -> RobotDirection {
        use RobotDirection::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn rotate_right(self) -> RobotDirection {
        use RobotDirection::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct RobotPosition(isize, isize);

impl RobotPosition {
    pub fn move_forward(self, direction: RobotDirection) -> Self {
        use RobotDirection::*;
        match direction {
            Up => RobotPosition(self.0, self.1 + 1),
            Right => RobotPosition(self.0 + 1, self.1),
            Down => RobotPosition(self.0, self.1 - 1),
            Left => RobotPosition(self.0 - 1, self.1),
        }
    }
}

