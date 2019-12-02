pub struct IntcodeComputer {
    pub memory: Vec<usize>,
    instruction_pointer: usize,
    halted: bool,
}

impl IntcodeComputer {
    pub fn new(memory: Vec<usize>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
            halted: false,
        }
    }

    pub fn step(&mut self) {
        match self.get_op() {
            Op::Add {
                src_addr_a,
                src_addr_b,
                dst_addr,
            } => {
                self.memory[dst_addr] = self.memory[src_addr_a] + self.memory[src_addr_b];
            }
            Op::Mult {
                src_addr_a,
                src_addr_b,
                dst_addr,
            } => {
                self.memory[dst_addr] = self.memory[src_addr_a] * self.memory[src_addr_b];
            }
            Op::Halt => self.halted = true,
        }
        self.instruction_pointer += 4;
    }

    pub fn run_to_end(&mut self) {
        while !self.halted {
            self.step()
        }
    }

    fn read_relative(&self, offset: usize) -> usize {
        let pc = self.instruction_pointer;
        self.memory[pc + offset]
    }

    fn get_op(&self) -> Op {
        let opcode = self.memory[self.instruction_pointer];
        match opcode {
            1 => Op::Add {
                src_addr_a: self.read_relative(1),
                src_addr_b: self.read_relative(2),
                dst_addr: self.read_relative(3),
            },
            2 => Op::Mult {
                src_addr_a: self.read_relative(1),
                src_addr_b: self.read_relative(2),
                dst_addr: self.read_relative(3),
            },
            99 => Op::Halt,
            _ => panic!(format!("Unknown op code {}", opcode)),
        }
    }
}

enum Op {
    Add {
        src_addr_a: usize,
        src_addr_b: usize,
        dst_addr: usize,
    },
    Mult {
        src_addr_a: usize,
        src_addr_b: usize,
        dst_addr: usize,
    },
    Halt,
}

#[cfg(test)]
mod tests {
    use super::IntcodeComputer;

    #[test]
    fn example_1() {
        let mut computer = IntcodeComputer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
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
        assert!(computer.halted);
    }

    #[test]
    fn example_2() {
        let mut computer = IntcodeComputer::new(vec![1, 0, 0, 0, 99]);
        computer.run_to_end();
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example_3() {
        let mut computer = IntcodeComputer::new(vec![2, 4, 4, 5, 99, 0]);
        computer.run_to_end();
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example_4() {
        let mut computer = IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run_to_end();
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
