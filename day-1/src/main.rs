fn day_1(summaries: Vec<u32>) {
    println!("{}", summaries.iter().max().expect("vec is empty"));
}

fn day_2(mut summaries: Vec<u32>) {
    summaries.sort();
    println!("{}", summaries.into_iter().rev().take(3).sum::<u32>());
}

fn main() {
    let input = include_str!("input.txt");
    let summaries = input
        .split("\n\n")
        .map(|chunk| -> u32 {
            chunk
                .lines()
                .map(|string_line| string_line.parse::<u32>().expect("line should be a number"))
                .sum()
        })
        .collect::<Vec<u32>>();

    day_1(summaries.clone());
    day_2(summaries);
}
