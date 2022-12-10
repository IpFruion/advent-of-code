use super::instruction::Instruction;

#[derive(Debug)]
pub struct Cpu {
    cycle: usize,
    x: i64,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu { cycle: 0, x: 1 }
    }
}

impl Cpu {
    pub fn run_signal(&mut self, instruction: Instruction) -> i64 {
        let mut signal_strength = 0;
        let cycles = instruction.cycles();
        // the cycle begins (zero indexed)
        for _ in 0..cycles {
            //during cycle
            if let Some(cyc) = (self.cycle + 1).checked_sub(20) {
                if cyc % 40 == 0 {
                    signal_strength = (self.cycle + 1) as i64 * self.x;
                }
            }
            // end of cycle
            self.cycle += 1;
        }

        match instruction {
            Instruction::AddX(v) => self.x += v,
            _ => {}
        }

        signal_strength
    }

    /// Determines if the pixel should be lit up or not
    pub fn draw_instruction(&mut self, instruction: Instruction, crt: &mut String) {
        let cycles = instruction.cycles();

        for _ in 0..cycles {
            let rem = self.cycle % 40;
            // println!("{} | {}", self.x, rem);
            if self.x - 1 <= rem as i64 && rem as i64 <= self.x + 1 {
                crt.push('#');
            } else {
                crt.push('.')
            }
            if (self.cycle + 1) % 40 == 0 {
                crt.push('\n');
            }
            self.cycle += 1
        }
        match instruction {
            Instruction::AddX(v) => self.x += v,
            _ => {}
        }
    }
}
