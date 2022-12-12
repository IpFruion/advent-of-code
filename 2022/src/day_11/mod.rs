use std::collections::{BTreeMap, HashMap};

use advent_of_code::errors::{Error, Result};

use self::monkey::Monkey;

mod monkey;
mod operation;
mod test;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let mut lines = lines.filter(|l| !l.as_ref().trim().is_empty());
    let mut monkies = BTreeMap::new();
    let mut visited = HashMap::new();

    while let Ok(monkey) = Monkey::from_lines(&mut lines) {
        visited.insert(monkey.index, 0);
        monkies.insert(monkey.index, monkey);
    }

    // run rounds
    for _ in 0..1 {
        let indices: Vec<usize> = monkies.keys().copied().collect();
        for i in indices {
            let monkey = monkies.get_mut(&i).unwrap();
            let thrown = monkey.get_thrown().unwrap();
            let active = visited.get_mut(&monkey.index).unwrap();
            *active += thrown.len();
            for (item, to) in thrown {
                monkies.get_mut(&to).unwrap().add_item(item);
            }
            println!("{}", i);
            for (_, m) in monkies.iter() {
                println!("\t{}", m);
            }
        }
    }

    let mut active: Vec<usize> = visited.into_iter().map(|(_, v)| v).collect();

    active.sort_unstable();
    active.reverse();
    let active = active[0] * active[1];

    Ok(active)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let mut lines = lines.filter(|l| !l.as_ref().trim().is_empty());
    let mut monkies = BTreeMap::new();
    let mut visited = BTreeMap::new();

    while let Ok(monkey) = Monkey::from_lines(&mut lines) {
        visited.insert(monkey.index, 0);
        monkies.insert(monkey.index, monkey);
    }

    let mut common_denom = 1;

    for (_, m) in monkies.iter() {
        common_denom *= m.test.divisor as i64;
    }

    // run rounds
    for _ in 0..10_000 {
        let indices: Vec<usize> = monkies.keys().copied().collect();
        for i in indices {
            let monkey = monkies.get_mut(&i).unwrap();
            let thrown = monkey.get_thrown_without_worry(common_denom).unwrap();
            let active = visited.get_mut(&monkey.index).unwrap();
            *active += thrown.len();
            for (item, to) in thrown {
                monkies.get_mut(&to).unwrap().add_item(item);
            }

            // println!("{}", i);
            // for (_, m) in monkies.iter() {
            //     println!("{}", m);
            // }
        }
    }

    let mut active: Vec<usize> = visited.into_iter().map(|(_, v)| v).collect();

    active.sort_unstable();
    active.reverse();
    let active = active[0] * active[1];

    Ok(active)
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();
        assert_eq!(actual, 10605)
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt2(lines).unwrap();
        assert_eq!(actual, 2713310158)
    }
}
