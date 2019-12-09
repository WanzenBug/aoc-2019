use std::{
    error::Error,
    collections::VecDeque,
};

const INPUT: &'static str = include_str!("../INPUT");

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn decode(num: isize) -> Self {
        use ParameterMode::*;
        match num {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown parameter mode {}", num),
        }
    }

    fn fetch(&self, param: isize, base_ptr: isize, mem: &[isize]) -> isize {
        use ParameterMode::*;
        match self {
            &Position => mem[param as usize],
            &Relative => mem[(base_ptr + param) as usize],
            &Immediate => param,
        }
    }

    fn fetch_addr(&self, param: isize, base_ptr: isize) -> isize {
        use ParameterMode::*;
        match self {
            &Position => param,
            &Relative => base_ptr + param,
            &Immediate => panic!("Unsupported fetching of address in immediate mode"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: (ParameterMode, isize),
    },
    Mul {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: (ParameterMode, isize),
    },
    Input { dest_pos: (ParameterMode, isize) },
    Output { inp_pos: (ParameterMode, isize) },
    JumpIfTrue { bool_param: (ParameterMode, isize), jump_dest: (ParameterMode, isize) },
    JumpIfFalse { bool_param: (ParameterMode, isize), jump_dest: (ParameterMode, isize) },
    LessThan {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: (ParameterMode, isize),
    },
    Equals {
        left_op: (ParameterMode, isize),
        right_op: (ParameterMode, isize),
        dest_pos: (ParameterMode, isize),
    },
    SetRelativeOffset { source: (ParameterMode, isize) },
    Halt,
}

enum EvalResult {
    Halt,
    Continue,
    SetInstructionPtr(usize),
    UpdateRelativeOffset(isize),
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
            SetRelativeOffset { .. } => 2,
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
                let dmode = ParameterMode::decode((mem[0] / 10_000) % 10);
                Add {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: (dmode, mem[3]),
                }
            }
            2 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);
                let dmode = ParameterMode::decode((mem[0] / 10_000) % 10);

                Mul {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: (dmode, mem[3]),
                }
            }
            3 => {
                let dmode = ParameterMode::decode((mem[0] / 100) % 10);

                Input { dest_pos: (dmode, mem[1]) }
            }
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
                let dmode = ParameterMode::decode((mem[0] / 10_000) % 10);
                LessThan {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: (dmode, mem[3]),
                }
            }
            8 => {
                let lmode = ParameterMode::decode((mem[0] / 100) % 10);
                let rmode = ParameterMode::decode((mem[0] / 1000) % 10);
                let dmode = ParameterMode::decode((mem[0] / 10_000) % 10);
                Equals {
                    left_op: (lmode, mem[1]),
                    right_op: (rmode, mem[2]),
                    dest_pos: (dmode, mem[3]),
                }
            }
            9 => {
                let smode = ParameterMode::decode((mem[0] / 100) % 10);

                SetRelativeOffset {
                    source: (smode, mem[1]),
                }
            }
            99 => Halt,
            x => panic!("Unknown instruction {}", x),
        }
    }

    fn eval(self, mem: &mut [isize], base_ptr: isize) -> EvalResult {
        use Operation::*;
        match self {
            Add { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let (dmode, dval) = dest_pos;
                let dest_pos = dmode.fetch_addr(dval, base_ptr);
                let new_val = lmode.fetch(lparam, base_ptr, mem) + rmode.fetch(rparam, base_ptr, mem);
                mem[dest_pos as usize] = new_val;
                EvalResult::Continue
            }
            Mul { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let (dmode, dval) = dest_pos;
                let dest_pos = dmode.fetch_addr(dval, base_ptr);
                let new_val = lmode.fetch(lparam, base_ptr, mem) * rmode.fetch(rparam, base_ptr, mem);
                mem[dest_pos as usize] = new_val;
                EvalResult::Continue
            }
            Input { dest_pos } => {
                let (dmode, dval) = dest_pos;
                EvalResult::InputAt(dmode.fetch_addr(dval, base_ptr) as usize)
            }
            Output { inp_pos: dest_pos } => {
                let (dmode, dparam) = dest_pos;
                EvalResult::Output(dmode.fetch(dparam, base_ptr, mem))
            }
            Halt => EvalResult::Halt,
            JumpIfTrue { bool_param, jump_dest } => {
                let (bmode, baddr) = bool_param;
                if bmode.fetch(baddr, base_ptr, mem) != 0 {
                    let (jmode, jaddr) = jump_dest;
                    EvalResult::SetInstructionPtr(jmode.fetch(jaddr, base_ptr, mem) as usize)
                } else {
                    EvalResult::Continue
                }
            }
            JumpIfFalse { bool_param, jump_dest } => {
                let (bmode, baddr) = bool_param;
                if bmode.fetch(baddr, base_ptr, mem) == 0 {
                    let (jmode, jaddr) = jump_dest;
                    EvalResult::SetInstructionPtr(jmode.fetch(jaddr, base_ptr, mem) as usize)
                } else {
                    EvalResult::Continue
                }
            }
            LessThan { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let (dmode, dval) = dest_pos;
                let dest_pos = dmode.fetch_addr(dval, base_ptr);
                let new_val = lmode.fetch(lparam, base_ptr, mem) < rmode.fetch(rparam, base_ptr, mem);
                mem[dest_pos as usize] = if new_val { 1 } else { 0 };
                EvalResult::Continue
            }
            Equals { left_op, right_op, dest_pos } => {
                let (lmode, lparam) = left_op;
                let (rmode, rparam) = right_op;
                let (dmode, dval) = dest_pos;
                let dest_pos = dmode.fetch_addr(dval, base_ptr);
                let new_val = lmode.fetch(lparam, base_ptr, mem) == rmode.fetch(rparam, base_ptr, mem);
                mem[dest_pos as usize] = if new_val { 1 } else { 0 };
                EvalResult::Continue
            }
            SetRelativeOffset { source } => {
                let (smode, sval) = source;
                let new_val = smode.fetch(sval, base_ptr, mem);
                EvalResult::UpdateRelativeOffset(new_val)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT, &[2])?;
    eprintln!("result = {:#?}", result);

    Ok(())
}

fn run(input: &str, input_func: &[isize]) -> Result<VecDeque<isize>, Box<dyn Error + 'static>> {
    let memory: Result<Vec<isize>, _> = input.split(',')
        .map(|part| part.trim().parse::<isize>())
        .collect();
    let mut memory = memory?;

    memory.extend((0..1_000_000).map(|_| 0));

    let mut prog = Program { memory, instruction_ptr: 0, relative_offset: 0 };
    let mut input = input_func.iter().cloned().collect();
    let mut output = VecDeque::new();
    prog.run_all(&mut input, &mut output);
    Ok(output)
}


enum ProgramState {
    AwaitInput,
    Halt,
}

struct Program {
    memory: Vec<isize>,
    instruction_ptr: usize,
    relative_offset: isize,
}

impl Program {
    fn run(&mut self, mut input: Option<isize>) -> (ProgramState, Vec<isize>, Option<isize>) {
        let mut outputs = Vec::new();
        while self.instruction_ptr < self.memory.len() {
            let op = Operation::decode(&self.memory[self.instruction_ptr..]);
            let op_size = op.size();
            match op.eval(&mut self.memory, self.relative_offset) {
                EvalResult::Continue => self.instruction_ptr += op_size,
                EvalResult::SetInstructionPtr(x) => self.instruction_ptr = x,
                EvalResult::UpdateRelativeOffset(x) => {
                    self.relative_offset += x;
                    self.instruction_ptr += op_size;
                }
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
    fn test_all1() {
        let inp = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let result = run(inp, &[]).unwrap();
        assert_eq!(&result, &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
    }

    #[test]
    fn test_all2() {
        let inp = "1102,34915192,34915192,7,4,7,99,0";
        let result = run(inp, &[]).unwrap();
        assert_eq!(&result, &[1_219_070_632_396_864]);
    }


    #[test]
    fn test_all3() {
        let inp = "104,1125899906842624,99";
        let result = run(inp, &[]).unwrap();
        assert_eq!(&result, &[1125899906842624]);
    }

    #[test]
    fn test_instruction() {
        let inp = "109,-1,203,1,4,0,99";
        let result = run(inp, &[-70]).unwrap();
        assert_eq!(&result, &[-70]);
    }
}

