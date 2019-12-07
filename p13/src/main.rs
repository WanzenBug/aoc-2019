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

fn get_all_permutations() -> Vec<[isize; 5]> {
    let mut init = [0, 0, 0, 0, 0];
    let mut res = Vec::new();

    for i in 0..5 {
        init[0] = i;

        for j in (0..5).filter(|x| ![i].contains(x)) {
            init[1] = j;

            for k in (0..5).filter(|x| ![i, j].contains(x)) {
                init[2] = k;

                for l in (0..5).filter(|x| ![i, j, k].contains(x)) {
                    init[3] = l;

                    for m in (0..5).filter(|x| ![i, j, k, l].contains(x)) {
                        init[4] = m;

                        res.push(init.clone());
                    }
                }
            }
        }
    }
    res
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT)?;
    eprintln!("result = {:#?}", result);

    Ok(())
}

fn run(input: &str) -> Result<isize, Box<dyn Error + 'static>> {
    let memory: Result<Vec<isize>, _> = input.split(',')
        .map(|part| part.trim().parse::<isize>())
        .collect();
    let memory = memory?;

    let mut max = isize::min_value();
    for thruster_order in get_all_permutations() {
        let mut input_signal = 0;
        for phase in thruster_order.iter() {
            let mut mem_clone = memory.clone();
            input_signal = run_thruster_program(&mut mem_clone, *phase, input_signal)
        }

        if input_signal > max {
            max = input_signal
        }
    }

    Ok(max)
}

fn run_thruster_program(mem: &mut [isize], phase: isize, input_signal: isize) -> isize {
    let mut instruction_ptr = 0;
    let mut output = None;

    let inputs = [phase, input_signal];
    let mut inputs_iter = inputs.iter();
    let mut inp = || *inputs_iter.next().expect("Called input more than 2 times");
    let mut out = |out| output = Some(out);
    while instruction_ptr < mem.len() {
        let op = Operation::decode(&mem[instruction_ptr..]);
        let op_size = op.size();
        match op.eval(mem, &mut inp, &mut out) {
            EvalResult::Continue => instruction_ptr += op_size,
            EvalResult::SetInstructionPtr(x) => instruction_ptr = x,
            EvalResult::Halt => break,
        }
    }

    output.expect("Output must be set")
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
    fn test_all1() {
        let inp = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(run(inp).unwrap(), 43210);
    }

    #[test]
    fn test_all2() {
        let inp = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(run(inp).unwrap(), 54321);
    }

    #[test]
    fn test_all3() {
        let inp = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(run(inp).unwrap(), 65210);
    }


    #[test]
    fn test_permutations() {
        let perms = get_all_permutations();
        assert_eq!(perms.len(), 120);
        for p in perms {
            let mut dedup = p.to_vec();
            dedup.dedup();
            assert_eq!(dedup.len(), 5);
        }
    }
}
