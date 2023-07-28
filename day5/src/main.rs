const STACK_SIZE: usize = 9;

type Stacks = [Vec<char>; STACK_SIZE];
type Instructions = Vec<(usize, usize, usize)>;

fn puzzle_a(mut stacks: Stacks, instructions: &Instructions) -> String {
    instructions.iter().for_each(|(count, a, b)| {
        for _ in 0..*count {
            let c = stacks[a - 1].pop().unwrap();
            stacks[b - 1].push(c);
        }
    });

    stacks
        .map(|mut stack| stack.pop().unwrap().to_string())
        .join("")
}

fn puzzle_b(mut stacks: Stacks, instructions: &Instructions) -> String {
    instructions.iter().for_each(|(count, a, b)| {
        let stack = &mut stacks[a - 1];
        let last_idx = stack.len();

        let idx = if count > &last_idx {
            0
        } else {
            last_idx - count
        };

        let to_move = stack.splice(idx..last_idx, []);
        to_move
            .collect::<Vec<_>>()
            .iter()
            .for_each(|c| stacks[b - 1].push(*c));
    });

    stacks
        .map(|mut stack| stack.pop().unwrap().to_string())
        .join("")
}

fn main() {
    let input = include_str!("input.txt").trim_end();
    let (a, b) = input.split_once("\n\n").unwrap();
    let instructions: Instructions = b
        .lines()
        .map(|line| {
            let mut iter = line.split(" ").skip(1).step_by(2);
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            let c = iter.next().unwrap().parse().unwrap();
            (a, b, c)
        })
        .collect();

    let mut stacks: Stacks = Default::default();

    a.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push(c);
                }
            })
    });

    let answer_a = puzzle_a(stacks.clone(), &instructions);
    println!("Answer A: {}", answer_a);

    let answer_b = puzzle_b(stacks, &instructions);
    println!("Answer B: {}", answer_b)
}
