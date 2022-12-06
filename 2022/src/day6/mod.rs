use self::marker::Marker;

mod marker;

pub fn solution(length: usize, data_stream: &str) -> Option<usize> {
    let mut m = Marker::new(length);
    data_stream
        .chars()
        .enumerate()
        .skip_while(|(_, c)| !matches!(m.push(*c), Some(true)))
        .next()
        .map(|(i, _)| i + 1)
}

#[cfg(test)]
mod tests {
    use super::solution;

    const PAGE_EXAMPLE_1: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const PAGE_EXAMPLE_2: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const PAGE_EXAMPLE_3: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const PAGE_EXAMPLE_4: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const PAGE_EXAMPLE_5: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn page_example_1() {
        let tests = [
            (PAGE_EXAMPLE_1, 7),
            (PAGE_EXAMPLE_2, 5),
            (PAGE_EXAMPLE_3, 6),
            (PAGE_EXAMPLE_4, 10),
            (PAGE_EXAMPLE_5, 11),
        ];
        for (test, expected) in tests {
            let actual = solution(4, test).unwrap();
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn page_example_2() {
        let tests = [
            (PAGE_EXAMPLE_1, 19),
            (PAGE_EXAMPLE_2, 23),
            (PAGE_EXAMPLE_3, 23),
            (PAGE_EXAMPLE_4, 29),
            (PAGE_EXAMPLE_5, 26),
        ];
        for (test, expected) in tests {
            let actual = solution(14, test).unwrap();
            assert_eq!(actual, expected)
        }
    }
}
