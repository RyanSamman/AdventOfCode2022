use std::fs::read_to_string;


#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

fn process_ops(ops: Vec<Operation>) -> Vec<i32> {
    let mut x = 1;
    let mut x_history = Vec::new();

    x_history.push(x);

    for op in ops {
        match op {
            Operation::Noop => {
                x_history.push(x);
            },
            Operation::Addx(dx) => {
                x_history.push(x);
                x_history.push(x);
                x = x + dx;
            },
        }
    }

    x_history
}

fn parse_ops(filename: &str) -> Vec<Operation> {
    let input = read_to_string(filename).unwrap();
    let mut ops = Vec::new();

    for line in input.lines() {
        let op = match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => Operation::Noop,
            ["addx", n] if n.parse::<i32>().is_ok() => Operation::Addx(n.parse().unwrap()),
            _ => panic!("Invalid operation: {line}"),
        };

        ops.push(op);
    }

    ops
}

fn main() {
    let ops = parse_ops("src/input.txt");

    let cycles: &[usize] = &[20, 60, 100, 140, 180, 220];

    let history = process_ops(ops);

    let h = history.iter().enumerate().for_each(|(i, x)| println!("Cycle {i}: {x}"));

    let signal: i32 = cycles.iter()
                            .map(|&c| (c as i32) * history[c])
                            .sum();

    for i in 0..6 {
        for j in 0..40 {
            let x = history[i * 40 + j+ 1] as usize;

            if x - 1 <= j && j <= x + 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!("{signal}");
}
