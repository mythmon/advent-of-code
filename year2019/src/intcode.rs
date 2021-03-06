use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

#[allow(clippy::clippy::module_name_repetitions)]
pub struct IntcodeComputer {
    memory: Vec<isize>,
    pub output: Vec<isize>,
    output_pointer: usize,
    input: Vec<isize>,
    input_pointer: usize,
    instruction_pointer: isize,
    state: ComputerState,
    verbose: bool,
    relative_base: isize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComputerState {
    Ready,
    Halted,
    WaitingForInput,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PauseReason {
    Halt,
    Output(isize),
    Input,
}

// public methods
impl IntcodeComputer {
    #[must_use]
    pub fn build(initial_memory: Vec<isize>) -> IntcodeComputerBuilder {
        IntcodeComputerBuilder::new(initial_memory)
    }

    pub fn step(&mut self) {
        let op = self.get_op();
        self.log(format!("[{:>4}] {}", self.instruction_pointer, &op));
        let mut should_advance_ip = true;
        match op {
            Op::Add { src_a, src_b, dst } => {
                self.write_mem(
                    self.get_param_addr(dst),
                    self.get_param(src_a) + self.get_param(src_b),
                );
            }
            Op::Mult { src_a, src_b, dst } => {
                let a = self.get_param(src_a);
                let b = self.get_param(src_b);
                let dst = self.get_param_addr(dst);
                self.write_mem(dst, a * b);
            }
            Op::Input { dst } => {
                if let Some(input) = self.get_input() {
                    let dst = self.get_param_addr(dst);
                    self.write_mem(dst, input);
                } else {
                    self.state = ComputerState::WaitingForInput;
                    should_advance_ip = false;
                }
            }
            Op::Output { src } => {
                self.output.push(self.get_param(src));
            }
            Op::JumpIfTrue { predicate, target } => {
                if self.get_param(predicate) != 0 {
                    should_advance_ip = false;
                    self.instruction_pointer = self.get_param(target) as isize;
                }
            }
            Op::JumpIfFalse { predicate, target } => {
                if self.get_param(predicate) == 0 {
                    should_advance_ip = false;
                    self.instruction_pointer = self.get_param(target) as isize;
                }
            }
            Op::LessThan { src_a, src_b, dst } => {
                self.write_mem(
                    self.get_param_addr(dst),
                    if self.get_param(src_a) < self.get_param(src_b) {
                        1
                    } else {
                        0
                    },
                );
            }
            Op::Equals { src_a, src_b, dst } => {
                self.write_mem(
                    self.get_param_addr(dst),
                    if self.get_param(src_a) == self.get_param(src_b) {
                        1
                    } else {
                        0
                    },
                );
            }
            Op::AdjustRelBase { src } => {
                self.relative_base += self.get_param(src);
            }
            Op::Halt => self.state = ComputerState::Halted,
        }
        if should_advance_ip {
            self.instruction_pointer += op.size();
        }
    }

    pub fn run_to_end(&mut self) {
        while self.state == ComputerState::Ready {
            self.step();
        }
        if self.state == ComputerState::WaitingForInput {
            panic!("Not enough input");
        }
    }

    pub fn run_until_io(&mut self) -> PauseReason {
        while self.state == ComputerState::Ready && self.output_pointer >= self.output.len() {
            self.step()
        }
        match self.state {
            ComputerState::Ready => {
                let rv = self.output[self.output_pointer];
                self.output_pointer += 1;
                PauseReason::Output(rv)
            }
            ComputerState::WaitingForInput => PauseReason::Input,
            ComputerState::Halted => PauseReason::Halt,
        }
    }

    pub fn add_input(&mut self, v: isize) {
        self.input.push(v);
        if self.state == ComputerState::WaitingForInput {
            self.state = ComputerState::Ready;
        }
    }
}

// private methods
impl IntcodeComputer {
    fn read_relative(&self, offset: isize) -> isize {
        let pc = self.instruction_pointer;
        self.read_mem(pc + offset)
    }

    fn get_param(&self, param: Parameter) -> isize {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Position(addr) => self.read_mem(addr),
            Parameter::Relative(offset) => {
                let addr = self.relative_base + offset;
                assert!(
                    addr >= 0,
                    "Invalid result of relative parameter read: {}",
                    addr
                );
                self.read_mem(addr)
            }
        }
    }

    fn get_param_addr(&self, param: Parameter) -> isize {
        match param {
            Parameter::Immediate(_) => panic!("Still can't write to immediate values"),
            Parameter::Position(addr) => addr,
            Parameter::Relative(offset) => self.relative_base + offset,
        }
    }

    fn get_op(&mut self) -> Op {
        let instruction = self.read_mem(self.instruction_pointer);
        let opcode = instruction % 100;
        let param_modes: [ParameterMode; 3] = [
            (instruction / 100 % 10)
                .try_into()
                .expect("Couldn't parse parameter mode"),
            (instruction / 1_000 % 10)
                .try_into()
                .expect("Couldn't parse parameter mode"),
            (instruction / 10_000 % 10)
                .try_into()
                .expect("Couldn't parse parameter mode"),
        ];

        match opcode {
            1 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                let dst = self.read_relative(3);
                assert_ne!(param_modes[2], ParameterMode::Immediate);
                Op::Add {
                    src_a: param_modes[0].with_value(a),
                    src_b: param_modes[1].with_value(b),
                    dst: param_modes[2].with_value(dst),
                }
            }
            2 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                let dst = self.read_relative(3);
                assert_ne!(param_modes[2], ParameterMode::Immediate);
                Op::Mult {
                    src_a: param_modes[0].with_value(a),
                    src_b: param_modes[1].with_value(b),
                    dst: param_modes[2].with_value(dst),
                }
            }
            3 => {
                let dst = self.read_relative(1);
                assert_ne!(param_modes[0], ParameterMode::Immediate);
                Op::Input {
                    dst: param_modes[0].with_value(dst),
                }
            }
            4 => {
                let a = self.read_relative(1);
                Op::Output {
                    src: param_modes[0].with_value(a),
                }
            }
            5 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                Op::JumpIfTrue {
                    predicate: param_modes[0].with_value(a),
                    target: param_modes[1].with_value(b),
                }
            }
            6 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                Op::JumpIfFalse {
                    predicate: param_modes[0].with_value(a),
                    target: param_modes[1].with_value(b),
                }
            }
            7 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                let dst = self.read_relative(3);
                assert_ne!(param_modes[2], ParameterMode::Immediate);
                Op::LessThan {
                    src_a: param_modes[0].with_value(a),
                    src_b: param_modes[1].with_value(b),
                    dst: param_modes[2].with_value(dst),
                }
            }
            8 => {
                let a = self.read_relative(1);
                let b = self.read_relative(2);
                let dst = self.read_relative(3);
                assert_ne!(param_modes[2], ParameterMode::Immediate);
                Op::Equals {
                    src_a: param_modes[0].with_value(a),
                    src_b: param_modes[1].with_value(b),
                    dst: param_modes[2].with_value(dst),
                }
            }
            9 => Op::AdjustRelBase {
                src: param_modes[0].with_value(self.read_relative(1)),
            },
            99 => Op::Halt,
            _ => panic!(format!("Unknown op code {}", opcode)),
        }
    }

    fn get_input(&mut self) -> Option<isize> {
        if self.input_pointer < self.input.len() {
            let rv = self.input[self.input_pointer];
            self.input_pointer += 1;
            Some(rv)
        } else {
            None
        }
    }

    fn log<M: fmt::Display>(&self, msg: M) {
        if self.verbose {
            println!("{}", msg);
        }
    }

    #[must_use]
    pub fn read_mem(&self, addr: isize) -> isize {
        assert!(addr >= 0, "invalid memory address");
        *self.memory.get(addr as usize).unwrap_or(&0)
    }

    pub fn write_mem(&mut self, addr: isize, val: isize) {
        assert!(addr >= 0, "invalid memory address");
        let addr = addr as usize;
        if addr >= self.memory.len() {
            self.memory.resize_with(addr + 1, || 0);
        }
        self.memory[addr] = val;
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct IntcodeComputerBuilder {
    initial_memory: Vec<isize>,
    input: Vec<isize>,
    verbose: bool,
}

impl IntcodeComputerBuilder {
    fn new(initial_memory: Vec<isize>) -> Self {
        Self {
            initial_memory,
            input: vec![],
            verbose: false,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // this can't be const because it consumes `self`
    pub fn done(self) -> IntcodeComputer {
        IntcodeComputer {
            memory: self.initial_memory,
            instruction_pointer: 0,
            state: ComputerState::Ready,
            input: self.input,
            input_pointer: 0,
            output: Vec::new(),
            output_pointer: 0,
            verbose: self.verbose,
            relative_base: 0,
        }
    }

    pub fn with_input<I: IntoIterator<Item = isize>>(mut self, input: I) -> Self {
        self.input.extend(input);
        self
    }

    #[allow(dead_code)]
    #[must_use]
    pub const fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add {
        src_a: Parameter,
        src_b: Parameter,
        dst: Parameter,
    },
    Mult {
        src_a: Parameter,
        src_b: Parameter,
        dst: Parameter,
    },
    Input {
        dst: Parameter,
    },
    Output {
        src: Parameter,
    },
    JumpIfTrue {
        predicate: Parameter,
        target: Parameter,
    },
    JumpIfFalse {
        predicate: Parameter,
        target: Parameter,
    },
    LessThan {
        src_a: Parameter,
        src_b: Parameter,
        dst: Parameter,
    },
    Equals {
        src_a: Parameter,
        src_b: Parameter,
        dst: Parameter,
    },
    Halt,
    AdjustRelBase {
        src: Parameter,
    },
}

impl fmt::Display for Op {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add { src_a, src_b, dst } => write!(fmt, "ADD {} {} &{}", src_a, src_b, dst)?,
            Self::Mult { src_a, src_b, dst } => write!(fmt, "MUL {} {} &{}", src_a, src_b, dst)?,
            Self::Input { dst } => write!(fmt, "INP &{}", dst)?,
            Self::Output { src } => write!(fmt, "INP {}", src)?,
            Self::Halt => write!(fmt, "HLT")?,
            Self::LessThan { src_a, src_b, dst } => {
                write!(fmt, "LST {} {} &{}", src_a, src_b, dst)?
            }
            Self::Equals { src_a, src_b, dst } => write!(fmt, "EQS {} {} &{}", src_a, src_b, dst)?,
            Self::JumpIfTrue { predicate, target } => write!(fmt, "JIT {} {}", predicate, target)?,
            Self::JumpIfFalse { predicate, target } => write!(fmt, "JIF {} {}", predicate, target)?,
            Self::AdjustRelBase { src } => write!(fmt, "ARB {}", src)?,
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Parameter {
    Immediate(isize),
    Position(isize),
    Relative(isize),
}

impl fmt::Display for Parameter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Immediate(v) => write!(fmt, "!{}", v)?,
            Self::Position(v) => write!(fmt, "&{}", v)?,
            Self::Relative(v) => write!(fmt, "~{}", v)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl ParameterMode {
    fn with_value(self, value: isize) -> Parameter {
        match self {
            ParameterMode::Position => {
                assert!(
                    value >= 0,
                    format!("Invalid memory address while building parameter: {}", value)
                );
                Parameter::Position(value as isize)
            }
            ParameterMode::Immediate => Parameter::Immediate(value),
            ParameterMode::Relative => Parameter::Relative(value),
        }
    }
}

impl TryFrom<isize> for ParameterMode {
    type Error = String;

    fn try_from(mode: isize) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(format!("Unknown parameter mode {}", mode)),
        }
    }
}

impl Op {
    const fn size(&self) -> isize {
        match self {
            Self::Add { .. } | Self::Mult { .. } | Self::LessThan { .. } | Self::Equals { .. } => 4,
            Self::JumpIfTrue { .. } | Self::JumpIfFalse { .. } => 3,
            Self::Input { .. } | Self::Output { .. } | Self::AdjustRelBase { .. } => 2,
            Self::Halt => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ComputerState, IntcodeComputer};

    #[test]
    fn day02_example1() {
        let mut computer =
            IntcodeComputer::build(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).done();
        computer.step();
        assert_eq!(
            computer.memory,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        computer.step();
        assert_eq!(
            computer.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        computer.step();
        assert!(computer.state == ComputerState::Halted);
    }

    #[test]
    fn day02_example2() {
        let mut computer = IntcodeComputer::build(vec![1, 0, 0, 0, 99]).done();
        computer.run_to_end();
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn day02_example3() {
        let mut computer = IntcodeComputer::build(vec![2, 4, 4, 5, 99, 0]).done();
        computer.run_to_end();
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day02_example4() {
        let mut computer = IntcodeComputer::build(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).done();
        computer.run_to_end();
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day05_p1_example1() {
        let mut computer = IntcodeComputer::build(vec![3, 0, 4, 0, 99])
            .with_input(vec![42])
            .done();
        computer.run_to_end();
        assert_eq!(computer.output, vec![42]);
    }

    #[test]
    fn day05_p1_example2() {
        let mut computer = IntcodeComputer::build(vec![1002, 4, 3, 4, 33]).done();
        computer.run_to_end();
        assert_eq!(computer.read_mem(4), 99);
    }

    #[test]
    fn day05_p1_example3() {
        let mut computer = IntcodeComputer::build(vec![1101, 100, -1, 4, 0]).done();
        computer.run_to_end();
        assert_eq!(computer.read_mem(4), 99);
    }

    #[test]
    fn day05_p2_example1() {
        // program that tests if its input is equal to 8, using position mode
        let builder = IntcodeComputer::build(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

        // run with 8 as input, expect 1 (true) as output
        let mut computer = builder.clone().with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);

        // run with not 8 as input, expect 0 (false) as output
        let mut computer = builder.with_input(vec![9]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);
    }

    #[test]
    fn day05_p2_example2() {
        // program that tests if its input is less than 8, using position mode
        let builder = IntcodeComputer::build(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

        // run with less than 8 as input, expect 1 (true) as output
        let mut computer = builder.clone().with_input(vec![7]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);

        // run with 8 as input, expect 0 (false) as output
        let mut computer = builder.with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);
    }

    #[test]
    fn day05_p2_example3() {
        // program that tests if its input is equal to 8, using immediate mode
        let builder = IntcodeComputer::build(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

        // run with 8 as input, expect 1 (true) as output
        let mut computer = builder.clone().with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);

        // run with not 8 as input, expect 0 (false) as output
        let mut computer = builder.with_input(vec![9]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);
    }

    #[test]
    fn day05_p2_example4() {
        // program that tests if its input is less than 8, using immediate mode
        let builder = IntcodeComputer::build(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

        // run with less than 8 as input, expect 1 (true) as output
        let mut computer = builder.clone().with_input(vec![7]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);

        // run with 8 as input, expect 0 (false) as output
        let mut computer = builder.with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);
    }

    #[test]
    fn day05_p2_example5() {
        // program that uses jump to return 0 if the input is zero, and 1 if it is
        // non-zero (position mode)
        let builder = IntcodeComputer::build(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);

        // run with 0 as input, expect 0 (false) as output
        let mut computer = builder.clone().with_input(vec![0]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);

        // run with non-zero as input, expect 1 (true) as output
        let mut computer = builder.with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);
    }

    #[test]
    fn day05_p2_example6() {
        // program that uses jump to return 0 if the input is zero, and 1 if it is
        // non-zero (immediate mode)
        let builder = IntcodeComputer::build(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

        // run with 0 as input, expect 0 (false) as output
        let mut computer = builder.clone().with_input(vec![0]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 0);

        // run with non-zero as input, expect 1 (true) as output
        let mut computer = builder.with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1);
    }

    #[test]
    fn day05_p2_example7() {
        // "a larger example" that outputs 999 for inputs below 8, 1000 for
        // inputs equal to 8, and 1001 for inputs greater than 8
        let builder = IntcodeComputer::build(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);

        // run with 7 as input, expect 999 as output
        let mut computer = builder.clone().with_input(vec![7]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 999);

        // run with 8 as input, expect 1000 as output
        let mut computer = builder.clone().with_input(vec![8]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1000);

        // run with 9 as input, expect 1001 as output
        let mut computer = builder.with_input(vec![9]).done();
        computer.run_to_end();
        assert_eq!(computer.output[0], 1001);
    }

    #[test]
    fn day09_example1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = IntcodeComputer::build(program.clone()).done();
        computer.run_to_end();
        assert_eq!(computer.output, program);
    }

    #[test]
    fn day09_example2() {
        let mut computer =
            IntcodeComputer::build(vec![1_102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0]).done();
        computer.run_to_end();
        let out = computer.output[0];
        assert_eq!(out.to_string().len(), 16);
    }

    #[test]
    fn day09_example3() {
        let program = vec![104, 1_125_899_906_842_624, 99];
        let mut computer = IntcodeComputer::build(program.clone()).done();
        computer.run_to_end();
        assert_eq!(computer.output, vec![program[1]]);
    }

    #[test]
    fn debug1() {
        let mut computer = IntcodeComputer::build(vec![204, 0, 99]).done();
        computer.run_to_end();
        assert_eq!(computer.output, vec![204]);
    }
}
