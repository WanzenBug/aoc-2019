use std::{
    error::Error,
};

const INPUT: &'static str = include_str!("../INPUT");

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add {
        left_pos: usize,
        right_pos: usize,
        dest_pos: usize,
    },
    Mul {
        left_pos: usize,
        right_pos: usize,
        dest_pos: usize,
    },
    Halt,
}


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let memory: Result<Vec<usize>, _> = INPUT.split(',')
        .map(|part| part.trim().parse::<usize>())
        .collect();
    let memory = memory?;

    let (noun, verb) = find_noun_and_verb(&memory);
    println!("{}", noun * 100 + verb);
    Ok(())
}

fn find_noun_and_verb(original_program: &Vec<usize>) -> (usize, usize) {
    let mut program = original_program.clone();
    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;

            run(&mut program);
            if program[0] == 19690720 {
                return (noun, verb)
            }

            // reset
            program.copy_from_slice(original_program);
        }
    }

    panic!("No noun-verb combination found!")
}

fn run(mem: &mut [usize]) {
    for pos in (0..mem.len()).step_by(4) {
        let op = decode(mem, pos);

        if !evaluate(op, mem) {
            break;
        }
    }
}

fn decode(mem: &[usize], cur_pos: usize) -> Operation {
    match mem[cur_pos] {
        1 => Operation::Add {
            left_pos: mem[cur_pos + 1],
            right_pos: mem[cur_pos + 2],
            dest_pos: mem[cur_pos + 3],
        },
        2 => Operation::Mul {
            left_pos: mem[cur_pos + 1],
            right_pos: mem[cur_pos + 2],
            dest_pos: mem[cur_pos + 3],
        },
        99 => Operation::Halt,
        _ => panic!("Could not decode {} at {}", mem[cur_pos], cur_pos)
    }
}

fn evaluate(operation: Operation, mem: &mut [usize]) -> bool {
    match operation {
        Operation::Add { left_pos, right_pos, dest_pos } => {
            let new_val = mem[left_pos] + mem[right_pos];
            mem[dest_pos] = new_val;
            true
        }
        Operation::Mul { left_pos, right_pos, dest_pos } => {
            let new_val = mem[left_pos] * mem[right_pos];
            mem[dest_pos] = new_val;
            true
        }
        Operation::Halt => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let inp = [1, 0, 0, 0, 99];
        assert_eq!(decode(&inp, 0), Operation::Add { left_pos: 0, right_pos: 0, dest_pos: 0 });
        assert_eq!(decode(&inp, 4), Operation::Halt);
    }

    #[test]
    fn test_run() {
        let mut inp1 = [1, 0, 0, 0, 99];
        run(&mut inp1);
        let mut inp2 = [2, 3, 0, 3, 99];
        run(&mut inp2);
        let mut inp3 = [2, 4, 4, 5, 99, 0];
        run(&mut inp3);
        let mut inp4 = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut inp4);

        assert_eq!(inp1, [2, 0, 0, 0, 99]);
        assert_eq!(inp2, [2, 3, 0, 6, 99]);
        assert_eq!(inp3, [2, 4, 4, 5, 99, 9801]);
        assert_eq!(inp4, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
