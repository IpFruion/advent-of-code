use advent_of_code::errors::Result;

use self::packet::PacketData;

mod packet;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let mut index = 0;
    let mut right_order = 0;
    let mut left_comparison: Option<PacketData> = None;
    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            left_comparison = None;
        } else {
            if let Some(left_comparison) = left_comparison.as_ref() {
                index += 1;
                let right_comparison = line.parse()?;
                if left_comparison > &right_comparison {
                    right_order += index;
                }
            } else {
                left_comparison = Some(line.parse()?);
            }
        }
    }
    Ok(right_order)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let divider_packets = ["[[2]]".parse()?, "[[6]]".parse()?];

    let mut packets: Vec<PacketData> = lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|line| line.as_ref().parse())
        .collect::<Result<_>>()?;

    packets.extend_from_slice(&divider_packets);

    packets.sort();
    packets.reverse();

    // for p in packets.iter() {
    //     println!("{}", p);
    // }

    Ok(packets
        .iter()
        .enumerate()
        .filter(|(_, p)| divider_packets.contains(p))
        .map(|(i, _)| i + 1)
        .product())
}

#[cfg(test)]
mod tests {

    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 13);
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 140);
    }
}
