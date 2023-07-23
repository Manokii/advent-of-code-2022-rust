// Id Table: Rock = 1, Paper = 1, Scissors = 2
fn puzzle_a(input: Vec<(i16, i16)>) -> i16 {
    input
        .iter()
        .map(|(a, b)| -> i16 {
            /*
             * Weapon is always scored
             * Rock/X = 1, Paper/Y = 2, Scissors/Z = 3
             **/
            let weapon_score = b + 1;

            /*
             * Game score is based on this table
             * Win = 6, Draw = 3, Lose = 0
             *
             * if we substract it by the enemy weapon, we get the possible values
             * You:Scissors vs Enemy:Rock ((2 + 1) - 0 = 3) = Lose
             * You:Scissors vs Enemy:Paper ((2 + 1) - 1 = 2) = Win
             * You:Scissors vs Enemy:Scissors ((2 + 1) - 2 = 1) = Draw
             *
             * You:Rock vs Enemy:Rock ((0 + 1) - 0 = 1) = Draw
             * You:Rock vs Enemy:Paper ((0 + 1) - 1 = 0) = Lose
             * You:Rock vs Enemy:Scissors ((0 + 1) - 2 = -1) = Win
             *
             * You:Paper vs Enemy:Rock ((1 + 1) - 0 = 2) = Win
             * You:Paper vs Enemy:Paper ((1 + 1) - 1 = 1) = Draw
             * You:Paper vs Enemy:Scissors ((1 + 1) - 2 = 0) = Lose
             *
             * Now the magical part, the `rem_euclid()`
             * normally when you wanted to get the remainder of a division
             * you would do something like this
             * -1 % 3 = -2
             *
             * Pretty normal right? but that doesn't sit well with our table above,4 / 36
             * a Win can be -1 or 2, how do we fix this?4 / 36
             *4 / 36
             * rem_euclid() to the rescue,
             * it calculates the least nonnegative remainder of a given divisor
             *
             * let's say we have a = -7 and b = 4
             *  if we're getting the remainder using modulo it'll be a % b = r (-7 % 4 = -3)
             *
             * what rem_euclid do is take the first quotient that overshoots `|a|`
             * then add a number to walkback to 7
             * 4 * 2 = 8 -----> b * (⌊a / b⌋ + 1)
             * -7 + 8 = |1| -----> (|a| + (b * (⌊a / b⌋ + 1))) = |r|
             * so our remainder is 1
             *
             * so, if we use rem_euclid() on our table above
             * You:Rock vs Enemy:Scissors ((0 + 1) - 2 = -1) = Win
             * We have: a = -1, b = 3
             * 3 * 1 = 3 -----> b * (⌊a / b⌋ + 1)
             * -1 + 3 = |2| -----> (a + (b * (⌊a / b⌋ + 1))) = |r|
             *
             **/

            let game_result_id = (1 + b - a).rem_euclid(3);
            let game_score = 3 * game_result_id;
            weapon_score + game_score
        })
        .sum()
}

fn puzzle_b(input: Vec<(i16, i16)>) -> i16 {
    input
        .iter()
        .map(|(a, b)| -> i16 {
            let weapon = (b + a + 2) % 3;
            let weapon_score = weapon + 1;
            let game_result_score = b * 3;

            weapon_score + game_result_score
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt").trim();
    let games = input
        .split("\n")
        .map(|line| -> (i16, i16) {
            let mut chars = line.chars();
            let a = chars.nth(0).unwrap();
            let b = chars.nth(1).unwrap();
            // A|X = 0, B|Y = 1, C|Z = 2
            (a as i16 - 'A' as i16, b as i16 - 'X' as i16)
        })
        .collect::<Vec<(i16, i16)>>();

    let answer_a = puzzle_a(games.clone());
    let answer_b = puzzle_b(games);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}
