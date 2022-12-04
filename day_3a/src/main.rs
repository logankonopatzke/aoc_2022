use std::collections::HashSet;

fn get_priority_char(item: char) -> u8 {
    if item.is_lowercase() {
        return item as u8 - 96;
    } else {
        return item as u8 - 38;
    }
}

fn intersect(first_ruck: &str, second_ruck: &str) -> Vec<char> {
    let first_ruck: HashSet<char> = first_ruck.chars().collect();
    let second_ruck: HashSet<char> = second_ruck.chars().collect();

    first_ruck.intersection(&second_ruck).map(|i| *i).collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let sum = input.lines().fold(0, |acc, x| {
        let middle = x.len() / 2;

        let first_ruck = &x[0..middle];
        let second_ruck = &x[middle..x.len()];
        let intersection = intersect(first_ruck, second_ruck);

        acc + intersection
            .iter()
            .fold(0, |prev, y| prev + get_priority_char(*y) as i32)
    });

    println!("{sum}");
}
