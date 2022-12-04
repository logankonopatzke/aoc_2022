fn main() {
    let input = include_str!("../input.txt");

    let mut idx = 0;

    let sum = input.lines().fold(0, |acc, x| {
        idx += 1;
        let comma = x.find(",").unwrap();
        let first = &x[0..comma];
        let second = &x[comma + 1..x.len()];

        let dash = first.find("-").unwrap();
        let first = (
            &first[0..dash].parse::<i32>().unwrap(),
            &first[dash + 1..first.len()].parse::<i32>().unwrap(),
        );

        let dash = second.find("-").unwrap();
        let second = (
            &second[0..dash].parse::<i32>().unwrap(),
            &second[dash + 1..second.len()].parse::<i32>().unwrap(),
        );

        let mut add = 0;
        if first.0 <= second.1 && first.1 >= second.0 {
            add = 1
        }

        acc + add
    });

    println!("{sum}");
}
