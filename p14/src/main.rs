use std::{
    error::Error,
    collections::VecDeque
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
    InputAt(usize),
    Output(isize),
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

    fn eval(self, mem: &mut [isize]) -> EvalResult {
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
                EvalResult::InputAt(dest_pos as usize)
            }
            Output { inp_pos: dest_pos } => {
                let (dmode, dparam) = dest_pos;
                EvalResult::Output(dmode.fetch(dparam, mem))
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

    for i in 5..10 {
        init[0] = i;

        for j in (5..10).filter(|x| ![i].contains(x)) {
            init[1] = j;

            for k in (5..10).filter(|x| ![i, j].contains(x)) {
                init[2] = k;

                for l in (5..10).filter(|x| ![i, j, k].contains(x)) {
                    init[3] = l;

                    for m in (5..10).filter(|x| ![i, j, k, l].contains(x)) {
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
        let signal = run_phase_permutation(&memory, thruster_order);
        if signal > max {
            max = signal
        }
    }

    Ok(max)
}


fn run_phase_permutation(program: &Vec<isize>, phase_init: [isize; 5]) -> isize {
    let mut input_a = VecDeque::new();
    let mut input_b = VecDeque::new();
    let mut input_c = VecDeque::new();
    let mut input_d = VecDeque::new();
    let mut input_e = VecDeque::new();

    input_a.push_back(phase_init[0]);
    input_b.push_back(phase_init[1]);
    input_c.push_back(phase_init[2]);
    input_d.push_back(phase_init[3]);
    input_e.push_back(phase_init[4]);

    input_a.push_back(0);

    let mut prog_a = Program { memory: program.clone(), instruction_ptr: 0 };
    let mut prog_b = Program { memory: program.clone(), instruction_ptr: 0 };
    let mut prog_c = Program { memory: program.clone(), instruction_ptr: 0 };
    let mut prog_d = Program { memory: program.clone(), instruction_ptr: 0 };
    let mut prog_e = Program { memory: program.clone(), instruction_ptr: 0 };

    loop {
        let mut progress = false;

        progress |= prog_a.run_all(&mut input_a, &mut input_b);
        progress |= prog_b.run_all(&mut input_b, &mut input_c);
        progress |= prog_c.run_all(&mut input_c, &mut input_d);
        progress |= prog_d.run_all(&mut input_d, &mut input_e);
        progress |= prog_e.run_all(&mut input_e, &mut input_a);

        if !progress {
            assert_eq!(input_a.len(), 1);
            assert!(input_b.is_empty());
            assert!(input_c.is_empty());
            assert!(input_d.is_empty());
            assert!(input_e.is_empty());

            return input_a.pop_front().expect("Checked for exactly 1 element");
        }
    }
}

enum ProgramState {
    AwaitInput,
    Halt,
}

struct Program {
    memory: Vec<isize>,
    instruction_ptr: usize,
}

impl Program {
    fn run(&mut self, mut input: Option<isize>) -> (ProgramState, Vec<isize>, Option<isize>) {
        let mut outputs = Vec::new();
        while self.instruction_ptr < self.memory.len() {
            let op = Operation::decode(&self.memory[self.instruction_ptr..]);
            let op_size = op.size();
            match op.eval(&mut self.memory) {
                EvalResult::Continue => self.instruction_ptr += op_size,
                EvalResult::SetInstructionPtr(x) => self.instruction_ptr = x,
                EvalResult::Halt => return (ProgramState::Halt, outputs, input),
                EvalResult::InputAt(pos) => {
                    match input.take() {
                        Some(x) => {
                            self.memory[pos] = x;
                            self.instruction_ptr += op_size;
                        }
                        None => return (ProgramState::AwaitInput, outputs, input)
                    }
                }
                EvalResult::Output(x) => {
                    outputs.push(x);
                    self.instruction_ptr += op_size
                }
            }
        }
        (ProgramState::AwaitInput, outputs, input)
    }

    fn run_all(&mut self, input_queue: &mut VecDeque<isize>, output_queue: &mut VecDeque<isize>) -> bool {
        let mut progress = false;
        loop {
            let (state, output, input) = self.run(input_queue.pop_front());
            progress |= !output.is_empty();
            output_queue.extend(output.into_iter());
            if let Some(i) = input {
                input_queue.push_front(i);
            }
            match (state, input_queue.is_empty()) {
                (ProgramState::Halt, _) => break,
                (ProgramState::AwaitInput, true) => break,
                _ => (),
            }
        }
        progress
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
    fn test_all1() {
        let inp = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(run(inp).unwrap(), 139629729);
    }

    #[test]
    fn test_all2() {
        let inp = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(run(inp).unwrap(), 18216);
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

