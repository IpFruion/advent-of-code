use advent_of_code::{errors::Result, strings::TrimInPlace};

use self::{cpu::Cpu, instruction::Instruction};

mod cpu;
mod instruction;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<i64> {
    let mut cpu: Cpu = Default::default();

    lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|line| {
            let line = line.as_ref().trim();
            let signal = cpu.run_signal(line.parse()?);
            // if signal > 0 {
            //     println!("{:?} -> {}", cpu, signal);
            // }

            Ok(signal)
        })
        .sum()
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<String> {
    let mut cpu: Cpu = Default::default();
    let mut crt_screen = String::new();

    for line in lines.filter(|l| !l.as_ref().trim().is_empty()) {
        let line = line.as_ref().trim();
        cpu.draw_instruction(line.parse()?, &mut crt_screen);
    }

    crt_screen.trim_in_place();
    Ok(crt_screen)
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
addx 15
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
noop
        "#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();
        assert_eq!(actual, 13140)
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt2(lines).unwrap();
        let expected = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;
        assert_eq!(actual, expected.trim())
    }
}
