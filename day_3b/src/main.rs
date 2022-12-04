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

    let mut groups: Vec<String> = Vec::new();
    groups.resize(input.lines().count() / 3, "".to_string());

    let mut idx = 0;
    for (i, element) in input.lines().enumerate() {
        groups[idx] += element;
        groups[idx] += "\n";

        let current = i + 1;
        if current % 3 == 0 {
            idx += 1;
        }
    }

    let sum = groups.iter().fold(0, |acc, x| {
        let mut iter = x.lines();
        let intersection = intersect(iter.next().unwrap(), iter.next().unwrap());

        let prev: String = intersection.into_iter().collect();
        let intersection = intersect(prev.as_str(), iter.next().unwrap());

        return acc
            + intersection
                .iter()
                .fold(0, |other, y| other + get_priority_char(*y) as i32);
    });

    println!("{sum}");
}
