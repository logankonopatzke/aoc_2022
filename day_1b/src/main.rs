fn main() {
    let input = include_str!("../input.txt");

    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut calories: Vec<i32> = groups
        .iter()
        .map(|group| {
            return group
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x);
        })
        .collect();

    calories.sort();
    calories.reverse();
    println!("{}", calories[0] + calories[1] + calories[2]);
}
