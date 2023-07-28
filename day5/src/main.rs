use std::collections::LinkedList;
const STACK_SIZE: usize = 9;

type Stacks = [LinkedList<char>; STACK_SIZE];
type Instructions = Vec<(usize, usize, usize)>;

fn puzzle_a(mut stacks: Stacks, instructions: &Instructions) -> String {
    instructions.iter().for_each(|(count, a, b)| {
        for _ in 0..*count {
            let c = stacks[a - 1].pop_back().unwrap();
            stacks[b - 1].push_back(c);
        }
    });

    stacks
        .map(|mut stack| stack.pop_back().unwrap().to_string())
        .join("")
}

fn puzzle_b(mut stacks: Stacks, instructions: &Instructions) -> String {
    instructions.iter().for_each(|(count, a, b)| {
        let mut to_reverse: Vec<char> = Default::default();

        for _ in 0..*count {
            let c = stacks[a - 1].pop_back().unwrap();
            to_reverse.push(c);
        }

        to_reverse.iter().rev().for_each(|c| {
            stacks[b - 1].push_back(*c);
        })
    });

    stacks
        .map(|mut stack| stack.pop_back().unwrap().to_string())
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
                    stacks[i].push_back(c);
                }
            })
    });

    let answer_a = puzzle_a(stacks.clone(), &instructions);
    println!("Answer A {:?}", answer_a);

    let answer_b = puzzle_b(stacks, &instructions);
    println!("Answer B {:?}", answer_b)
}
