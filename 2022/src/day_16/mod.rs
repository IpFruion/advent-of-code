use std::collections::{BTreeMap, HashSet, VecDeque};

use advent_of_code::errors::{Error, Result};

use self::graph::Graph;
mod graph;
mod valve;

fn parse_valve<S: AsRef<str>>(line: S) -> Result<(String, usize, Vec<String>)> {
    let line = line.as_ref().trim();
    let (valve_flow, tunnels) = line
        .split_once(';')
        .ok_or(Error::InvalidParseError("Can't get sections".to_owned()))?;

    let mut valve_flow = valve_flow.split_whitespace().skip(1);

    let name = valve_flow
        .next()
        .ok_or(Error::InvalidParseError("Can't get name".to_owned()))?
        .to_string();

    let mut valve_flow = valve_flow.skip(2);
    let rate = valve_flow
        .next()
        .ok_or(Error::InvalidParseError("Can't get rate".to_owned()))?
        .split_once('=')
        .ok_or(Error::InvalidParseError("Can't get rate".to_owned()))?
        .1
        .parse()?;

    let tunnels = tunnels
        .split(',')
        .map(|s| {
            s.split_whitespace()
                .last()
                .ok_or(Error::InvalidParseError("Can't get tunnel".to_owned()))
                .map(|n| n.to_string())
        })
        .collect::<Result<_>>()?;

    Ok((name, rate, tunnels))
}

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    // The vertices are the string segments and the edge is the flow rate (capacity)
    let mut g: Graph<String, usize> = Graph::new();

    for l in lines.filter(|l| !l.as_ref().trim().is_empty()) {
        let (name, flow, tunnels) = parse_valve(l)?;
        let v = g.add_vertex(name);
        for t in tunnels {
            let t = g.add_vertex(t);
            g.add_edge(&v, &t, flow);
        }
    }

    println!("{:?}", g);

    let s = g.vertex("AA".to_string()).ok_or("No start valve")?;

    let mut queue = VecDeque::new();
    let mut visited = BTreeMap::new();
    let mut paths = BTreeMap::new();

    queue.push_front(s.clone());

    while let Some(cur) = queue.pop_back() {
        if let Some(edges) = g.edges(&cur) {
            for (to, e) in edges.iter() {
                if !visited.contains_key(to) && !to.eq(&s) {
                    visited.insert(to.clone(), e.clone());
                    queue.push_front(to.clone());
                    paths.insert(to.clone(), s.clone());
                }
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::solution_pt1;

    const PAGE_EXAMPLE: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;
    #[test]
    fn page_example_1() {
        let actual = solution_pt1(PAGE_EXAMPLE.lines()).unwrap();
        assert_eq!(actual, 1651)
    }
}
