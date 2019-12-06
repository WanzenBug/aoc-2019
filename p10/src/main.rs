use std::{
    error::Error,
};

const INPUT: &'static str = include_str!("../INPUT");

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn decode(num: isize) -> Self {
        use ParameterMode::*;
        match num {
            0 => Position,
            1 => Immediate,
            _ => panic!("Unknown parameter mode {}", num),
        }
    }

    fn fetch(&self, param: isize, mem: &[isize]) -> isize {
        use ParameterMode::*;
        match self {
            &Position => mem[param as usize],
            &Immediate => param,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: isize,
    },
    Mul {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: isize,
    },
    Input { dest_pos: isize },
    Output { inp_pos: (ParameterMode, isize) },
    JumpIfTrue { bool_param: (ParameterMode, isize), jump_dest: (ParameterMode, isize) },
    JumpIfFalse { bool_param: (ParameterMode, isize), jump_dest: (ParameterMode, isize) },
    LessThan {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: isize,
    },
    Equals {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: isize,
    },
    Halt,
}

enum EvalResult {
    Halt,
    Continue,
    SetInstructionPtr(usize),
}

impl Operation {
    fn size(&self) -> usize {
        use Operation::*;
        match self {
            Add { .. } => 4,
            Mul { .. } => 4,
            Input { .. } => 2,
            Output { .. } => 2,
            JumpIfTrue { .. } => 3,
            JumpIfFalse { .. } => 3,
            LessThan { .. } => 4,
            Equals { .. } => 4,
            Halt => 1,
        }
    }

    fn decode(mem: &[isize]) -> Self {
        let op = mem[0] % 100;
        use Operation::*;
        match op {
            1 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);

                Add {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: mem[3],
                }
            }
            2 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);

                Mul {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: mem[3],
                }
            }
            3 => Input { dest_pos: mem[1] },
            4 => {
                let opmode = ParameterMode::decode((mem[0] / 100) % 10);

                Output { inp_pos: (opmode, mem[1]) }
            }
            5 => {
                let bmode = ParameterMode::decode((mem[0] / 100) % 10);
                let dmode = ParameterMode::decode((mem[0] / 1000) % 10);

                JumpIfTrue {
                    bool_param: (bmode, mem[1]),
                    jump_dest: (dmode, mem[2]),
                }
            }
            6 => {
                let bmode = ParameterMode::decode((mem[0] / 100) % 10);
                let dmode = ParameterMode::decode((mem[0] / 1000) % 10);

                JumpIfFalse {
                    bool_param: (bmode, mem[1]),
                    jump_dest: (dmode, mem[2]),
                }
            }
            7 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);

                LessThan {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: mem[3],
                }
            }
            8 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);

                Equals {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: mem[3],
                }
            }
            99 => Halt,
            x => panic!("Unknown instruction {}", x),
        }
    }

    fn eval<FI, FO>(self, mem: &mut [isize], mut input: FI, mut output: FO) -> EvalResult where FI: FnMut() -> isize, FO: FnMut(isize) -> () {
        use Operation::*;
        match self {
            Add { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let new_val = lmode.fetch(lparam, mem) + rmode.fetch(rparam, mem);
                mem[dest_pos as usize] = new_val;
                EvalResult::Continue
            }
            Mul { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let new_val = lmode.fetch(lparam, mem) * rmode.fetch(rparam, mem);
                mem[dest_pos as usize] = new_val;
                EvalResult::Continue
            }
            Input { dest_pos } => {
                mem[dest_pos as usize] = input();
                EvalResult::Continue
            }

            Output { inp_pos: dest_pos } => {
                let (dmode, dparam) = dest_pos;
                output(dmode.fetch(dparam, mem));
                EvalResult::Continue
            }
            Halt => EvalResult::Halt,
            JumpIfTrue { bool_param, jump_dest } => {
                let (bmode, baddr) = bool_param;
                if bmode.fetch(baddr, mem) != 0 {
                    let (jmode, jaddr) = jump_dest;
                    EvalResult::SetInstructionPtr(jmode.fetch(jaddr, mem) as usize)
                } else {
                    EvalResult::Continue
                }
            }
            JumpIfFalse { bool_param, jump_dest } => {
                let (bmode, baddr) = bool_param;
                if bmode.fetch(baddr, mem) == 0 {
                    let (jmode, jaddr) = jump_dest;
                    EvalResult::SetInstructionPtr(jmode.fetch(jaddr, mem) as usize)
                } else {
                    EvalResult::Continue
                }
            }
            LessThan { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let new_val = lmode.fetch(lparam, mem) < rmode.fetch(rparam, mem);
                mem[dest_pos as usize] = if new_val { 1 } else { 0 };
                EvalResult::Continue
            }
            Equals { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let new_val = lmode.fetch(lparam, mem) == rmode.fetch(rparam, mem);
                mem[dest_pos as usize] = if new_val { 1 } else { 0 };
                EvalResult::Continue
            }
        }
    }
}


fn main() -> Result<(), Box<dyn Error + 'static>> {
    let memory: Result<Vec<isize>, _> = INPUT.split(',')
        .map(|part| part.trim().parse::<isize>())
        .collect();
    let mut memory = memory?;

    run(&mut memory);
    println!("{}", memory[0]);
    Ok(())
}

fn run(mem: &mut [isize]) {
    let mut instruction_ptr = 0;

    let mut inp = || 5;
    let mut out = |debug| eprintln!("debug = {:#?}", debug);
    while instruction_ptr < mem.len() {
        let op = Operation::decode(&mem[instruction_ptr..]);
        let op_size = op.size();
        match op.eval(mem, &mut inp, &mut out) {
            EvalResult::Continue => instruction_ptr += op_size,
            EvalResult::SetInstructionPtr(x) => instruction_ptr = x,
            EvalResult::Halt => break,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let inp = [1001, 4, 3, 4, 99];
        assert_eq!(Operation::decode(&inp[0..]), Operation::Add { left_op: (ParameterMode::Position, 4), right_op: (ParameterMode::Immediate, 3), dest_pos: 4 });
        assert_eq!(Operation::decode(&inp[4..]), Operation::Halt);
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
