type DoubleRange = (u8, u8, u8, u8);

fn puzzle_a(lines: Vec<DoubleRange>) -> usize {
    lines
        .iter()
        .filter(|(a_min, a_max, b_min, b_max)| {
            b_min >= a_min && b_max <= a_max || b_max >= a_min && b_max <= a_max
        })
        .count()
}

fn puzzle_b(lines: Vec<DoubleRange>) -> usize {
    lines
        .iter()
        .filter(|(a_min, a_max, b_min, b_max)| {
            b_min >= a_min && b_min <= a_max
                || b_max >= a_min && b_max <= a_max
                || a_min >= b_min && a_min <= b_max
                || a_max >= b_min && a_max <= b_max
        })
        .count()
}

fn main() {
    let input = include_str!("input.txt").trim();
    let lines = input
        .split("\n")
        .map(|line| -> DoubleRange {
            let (a, b) = line.split_once(",").unwrap();
            let ((a_min, a_max), (b_min, b_max)) =
                (a.split_once("-").unwrap(), b.split_once("-").unwrap());
            (
                a_min.parse().unwrap(),
                a_max.parse().unwrap(),
                b_min.parse().unwrap(),
                b_max.parse().unwrap(),
            )
        })
        .collect::<Vec<DoubleRange>>();

    println!("Puzzle A: {}", puzzle_a(lines.clone()));
    println!("Puzzle B: {}", puzzle_b(lines));
}
