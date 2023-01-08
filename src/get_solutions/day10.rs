use std::collections::VecDeque;

use super::AOCSolutions; 

enum Instruction {
    NoOp, 
    AddX(i64, usize), // value and remaining cycles 
}

impl Instruction {
    const ADDX_CYCLES: usize = 2; 

    fn from_line(line: &str) -> Instruction {
        let mut line_itr = line.trim().split_whitespace().take(2); 
        match line_itr.next() {
            Some("noop") => return Instruction::NoOp, 
            Some("addx") => {
                let x = line_itr.next(); 
                if x.is_none() {
                    panic!(
                        "[day10::Instruction::from_line] Malformed `addx` instruction \"{}\": Expected 1 argument but 0 given.", 
                        line.trim()
                    ); 
                }
                let x = x.unwrap().parse::<i64>()
                    .expect(&format!(
                        "[day10::Instruction::from_line] Malformed `addx` instruction \"{}\": Cannot parse argument to `i64`.", 
                        line.trim()
                    ));
                return Instruction::AddX(x, Instruction::ADDX_CYCLES); 
            }, 
            _ => panic!("[day10::Instruction::from_line] Undefined control symbol in input \"{}\"", line.trim()), 
        }
    }
}

struct Processor {
    reg_x: i64, 
    cycle: u64, 
    reg_wrb: i64, // Writeback at start of next cycle
    staged_instructions: VecDeque<Instruction>, // FIFO
    finish_code: bool, 
}

impl Processor {
    pub fn new() -> Processor {
        Processor { 
            reg_x: 1, 
            cycle: 0, 
            reg_wrb: 0, 
            staged_instructions: VecDeque::with_capacity(3), 
            finish_code: true, 
        }
    }

    pub fn get_signal_strength(&self) -> (i64, bool) {
        match TryInto::<i64>::try_into(self.cycle) {
            Ok(cycle) => (cycle * self.reg_x, false), 
            Err(_) => (-1, true), 
        }
    } 

    pub fn run_cycle(&mut self) {
        self.reg_x += self.reg_wrb; 
        self.reg_wrb = 0;

        let front_mut_ref = self.staged_instructions.front_mut(); 
        match front_mut_ref {
            Some(Instruction::AddX(x, cycles)) => 
            match cycles {
                cycles if *cycles > Instruction::ADDX_CYCLES || *cycles == 0 => { // Invalid cycle count
                    panic!("[day10::Processor::run_cycle] Unexpected cycle count `{}` for `AddX` instruction", cycles); 
                }, 
                1 => { // Last cycle -- WRB
                    self.reg_wrb = *x; 
                    self.staged_instructions.pop_front(); 
                }, 
                _ => { // Other cycle -- computing...
                    *cycles -= 1; 
                }, 
            }, 
            Some(Instruction::NoOp) => {
                self.staged_instructions.pop_front(); 
            },
            None => self.finish_code = true, 
        }

        self.cycle += 1;
    }

    pub fn issue_instruction(&mut self, instruction: Instruction) {
        self.finish_code = false; 
        self.staged_instructions.push_back(instruction); 
    }

    pub fn finished_running_program(&self) -> bool {
        self.finish_code
    }

    pub fn get_cycle(&self) -> u64 {
        self.cycle
    }

    pub fn get_reg(&self) -> i64 {
        self.reg_x
    }
}

pub struct Day10; 

impl Day10 {
    const LINE_SIZE: usize = 40; 
    const LIT: char = '#';
    const DIM: char = '.'; 
}

impl AOCSolutions for Day10 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut processor = Processor::new(); 
        let mut line_itr = input.lines(); 
        let mut sig_strength_sum: i64 = 0;
        loop { // by cycle, 1 issue per cycle
            if let Some(line) = line_itr.next() {
                processor.issue_instruction(Instruction::from_line(line)); 
            }

            processor.run_cycle(); 
            if processor.get_cycle() >= 20 && (processor.get_cycle() - 20) % 40 == 0 {
                match processor.get_signal_strength() {
                    (incr, false) => sig_strength_sum += incr, // No overflow
                    (_, true) => panic!("[Day9::sig_strength_sum] Overflow error when computing signal strength"), 
                }
            }

            if processor.finished_running_program() { break; }
        }
        return Ok(sig_strength_sum); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut processor = Processor::new(); 
        let mut instr_file_itr = input.lines(); 
        let mut output_line = String::with_capacity(Day10::LINE_SIZE);

        loop {
            if let Some(instr_line) = instr_file_itr.next() {
                processor.issue_instruction(Instruction::from_line(instr_line)); 
            }

            processor.run_cycle(); 
            if processor.finished_running_program() { // Output immediately if processor is done
                println!("{}", output_line); 
                break; 
            }

            let curr_pos: i64 = output_line.len().try_into().unwrap(); 
            if i64::abs(curr_pos - processor.get_reg()) <= 1 {
                output_line.push(Day10::LIT); 
            } else {
                output_line.push(Day10::DIM); 
            }
            
            if output_line.len() == Day10::LINE_SIZE { // Check if scan line filled
                println!("{}", output_line); 
                output_line.clear(); 
            }
        }
        return Ok(1); 
    }
}

#[cfg(test)]
mod tests {
    use crate::get_solutions::AOCSolutions;

    use super::Day10;
    use super::Processor;
    use super::Instruction; 

    const SAMPLE_INPUT: &str = r"noop
addx 3
addx -5";

    const LARGE_SAMPLE_INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_processor() {
        let mut processor = Processor::new(); 
        let mut input_iter = SAMPLE_INPUT.lines(); 
        
        // Test by procedure
        assert!(processor.finished_running_program()); 

        // noop
        processor.issue_instruction(Instruction::from_line(input_iter.next().unwrap())); 
        processor.run_cycle(); 
        assert_eq!(processor.reg_x, 1);
        assert_eq!(processor.cycle, 1);
        assert!(processor.staged_instructions.is_empty()); 

        // 2 addx
        processor.issue_instruction(Instruction::from_line(input_iter.next().unwrap())); // addx 3
        processor.issue_instruction(Instruction::from_line(input_iter.next().unwrap())); // addx -5

        processor.run_cycle(); 
        assert_eq!(processor.staged_instructions.len(), 2); 
        assert_eq!(processor.reg_x, 1);
        assert_eq!(processor.cycle, 2); 

        processor.run_cycle(); 
        assert_eq!(processor.staged_instructions.len(), 1); 
        assert_eq!(processor.reg_x, 1);
        assert_eq!(processor.cycle, 3); // Finished addx 3, no WRB

        processor.run_cycle(); 
        assert_eq!(processor.reg_x, 4); // WRB for addx 3
        assert_eq!(processor.cycle, 4); 

        processor.run_cycle(); 
        assert_eq!(processor.reg_x, 4); 
        assert_eq!(processor.cycle, 5); // Finished addx -5, no WRB

        processor.run_cycle(); 
        assert_eq!(processor.reg_x, -1); // WRB for addx -5
        assert!(processor.finished_running_program()); 
    }

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day10::get_star_1(LARGE_SAMPLE_INPUT).unwrap(), 13140); 
    }

    #[test]
    fn test_get_star_2() {
        Day10::get_star_2(LARGE_SAMPLE_INPUT); 
    }
}