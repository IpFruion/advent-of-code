use std::{borrow::Cow, cmp::Ordering, fmt::Display, str::FromStr};

use advent_of_code::errors::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PacketData {
    Value(i32),
    List(Vec<PacketData>),
}

impl PacketData {
    /// Creates a clone on write which will clone the underlying data when it is mutably borrowed
    /// or ownership is needed of the vector of packetData.
    ///
    /// This won't do anything if the borrow is immutable which we need for our iterator
    ///
    /// TODO: understand other ways to accomplish `Cow` i.e. `Rc`
    pub fn into_list(&self) -> Cow<Vec<PacketData>> {
        match self {
            Self::Value(v) => Cow::Owned(Vec::from([PacketData::Value(*v)])),
            Self::List(v) => Cow::Borrowed(v),
        }
    }
}

impl FromStr for PacketData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut cur_value = String::new();
        let mut cur: Option<PacketData> = None;
        let mut stack: Vec<PacketData> = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if !cur_value.is_empty() {
                        cur = Some(PacketData::Value(cur_value.parse()?));
                        cur_value.clear();
                    }
                    if let Some(PacketData::List(last)) = stack.last_mut() {
                        if let Some(cur) = cur {
                            last.push(cur);
                        }
                    }
                    cur = stack.pop();
                }
                '[' => stack.push(PacketData::List(Vec::new())),
                ' ' => {}
                ',' => {
                    if !cur_value.is_empty() {
                        cur = Some(PacketData::Value(cur_value.parse()?));
                        cur_value.clear();
                    }
                    if let Some(PacketData::List(last)) = stack.last_mut() {
                        if let Some(cur) = cur {
                            last.push(cur);
                        }
                    }
                    cur = None;
                }
                c => cur_value.push(c),
            };
            // println!("({}) | {:?} | {:?} | ({})", c, cur, stack, cur_value);
            if stack.is_empty() {
                break;
            }
        }

        cur.ok_or(Error::InvalidParseError(
            "Couldn't get Packet Data for: ".to_owned() + s,
        ))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //TODO: non-recursive solution
        match (self, other) {
            // left < right means that self > other or left compared to right in the right order
            (Self::Value(left), Self::Value(right)) => left.cmp(right).reverse(),
            (left, right) => {
                let left = left.into_list();
                let right = right.into_list();
                let mut left = left.iter();
                let mut right = right.iter();

                loop {
                    match (left.next(), right.next()) {
                        (Some(left), Some(right)) => match left.cmp(right) {
                            Ordering::Equal => {}
                            o => return o,
                        },
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Greater,
                        (Some(_), None) => return Ordering::Less,
                    }
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::List(l) => {
                let mut s = String::from("[");
                for v in l {
                    s.push_str(&format!("{}", v));
                    s.push(',');
                }
                s.pop();
                s.push(']');
                write!(f, "{}", s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PacketData;

    #[test]
    fn invalid_packet() {
        let res = "".parse::<PacketData>();
        assert!(res.is_err())
    }

    #[test]
    fn empty_list_packet() {
        let res = "[]".parse::<PacketData>().unwrap();
        assert!(matches!(res, PacketData::List(_)))
    }

    #[test]
    fn simple_packet() {
        let res = "[1, 2]".parse::<PacketData>().unwrap();
        match res {
            PacketData::List(l) => {
                assert_eq!(l, Vec::from([PacketData::Value(1), PacketData::Value(2)]))
            }
            _ => assert!(false),
        };
    }

    #[test]
    fn more_complex_packet() {
        let res = "[[0, 1], 2]".parse::<PacketData>().unwrap();
        let expected = PacketData::List(Vec::from([
            PacketData::List(Vec::from([PacketData::Value(0), PacketData::Value(1)])),
            PacketData::Value(2),
        ]));
        assert_eq!(res, expected);
    }
}
