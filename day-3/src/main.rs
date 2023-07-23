use std::str::Split;

fn puzzle_a(rucksacks: Split<&str>) -> u32 {
    rucksacks
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| -> u32 {
            b.chars()
                .filter(|char| a.contains(*char))
                .map(|char| -> u32 {
                    let charcode = u32::from(char);
                    // 'a' is greater than 'A'
                    if charcode >= u32::from('a') {
                        charcode - u32::from('a') + 1
                    } else {
                        charcode - u32::from('A') + 27
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}

fn puzzle_b(rucksacks: Split<&str>) -> u32 {
    rucksacks
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            group[0]
                .chars()
                .filter(|char| group[1].contains(*char) && group[2].contains(*char))
                .next()
                .unwrap()
        })
        .map(|char| -> u32 {
            let charcode = u32::from(char);
            if charcode >= u32::from('a') {
                charcode - u32::from('a') + 1
            } else {
                charcode - u32::from('A') + 27
            }
        })
        .sum()
}
fn main() {
    let input = include_str!("input.txt").trim();

    let games = input.split("\n");

    let answer_a = puzzle_a(games.clone());
    let answer_b = puzzle_b(games);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}
