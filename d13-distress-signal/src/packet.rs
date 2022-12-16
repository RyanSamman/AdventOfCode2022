use std::{cmp::Ordering, fmt::Display};

mod test {
    #[test]
    fn test_numbers() {
        use super::compare_number_packets;
        use super::PacketComparison::*;
        assert_eq!(compare_number_packets(&0, &0), KeepChecking);
        assert_eq!(compare_number_packets(&0, &1), CorrectOrder);
        assert_eq!(compare_number_packets(&1, &0), IncorrectOrder);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketComparison::*;

        match is_in_order(self, other) {
            CorrectOrder => Ordering::Less,
            IncorrectOrder => Ordering::Greater,
            KeepChecking => Ordering::Equal,
        }
    }
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match self {
            Packet::Number(i) => s.push_str(&i.to_string()),
            Packet::List(l) => {
                s.push('[');

                l.into_iter().map(|p| p.to_string()).for_each(|ps| {
                    s.push_str(&ps);
                    s.push(',');
                });


                if l.len() >= 1 {
                    s.pop();
                }

                s.push(']');
            }
        }

        s
    }
}

// If I were to redo this, change PacketComparison into Ord in the first place

// NOTE: Basically recreated an Ordering
#[derive(Debug, PartialEq)]
enum PacketComparison {
    CorrectOrder,   // Less than
    IncorrectOrder, // Greater
    KeepChecking,   // Equal
}

pub fn compare_packets(left: &Packet, right: &Packet) -> bool {
    if let PacketComparison::IncorrectOrder = is_in_order(left, right) {
        return false;
    } else {
        return true;
    }
}

fn compare_number_packets(i: &i32, j: &i32) -> PacketComparison {
    use PacketComparison::*;
    if i == j {
        return KeepChecking;
    } else if i < j {
        return CorrectOrder;
    } else {
        return IncorrectOrder;
    }
}

fn is_in_order(left: &Packet, right: &Packet) -> PacketComparison {
    use Packet::*;
    match (left, right) {
        (Number(l), Number(r)) => compare_number_packets(l, r),
        (List(l), List(r)) => is_lists_in_order(l, r),
        (List(ll), Number(rn)) => is_lists_in_order(ll, &vec![Number(*rn)]),
        (Number(ln), List(rl)) => is_lists_in_order(&vec![Number(*ln)], rl),
    }
}

fn is_lists_in_order(left: &Vec<Packet>, right: &Vec<Packet>) -> PacketComparison {
    use PacketComparison::*;

    for i in 0..left.len() {
        if i >= right.len() {
            return IncorrectOrder;
        }

        let state = is_in_order(&left[i], &right[i]);

        match state {
            KeepChecking => {}
            cmp => return cmp,
        }
    }

    if left.len() == right.len() {
        return KeepChecking;
    }

    return CorrectOrder;
}
