fn main() {
    let input = include_str!("../input.txt");

    let sum = input.lines().fold(0, |acc, x| {
        let (first, second) = x.split_once(",").unwrap();

        let first = first.split_once("-").unwrap();
        let second = second.split_once("-").unwrap();

        let first: (i32, i32) = (first.0.parse().unwrap(), first.1.parse().unwrap());
        let second: (i32, i32) = (second.0.parse().unwrap(), second.1.parse().unwrap());

        let mut add = 0;
        if (first.0 >= second.0 && first.1 <= second.1)
            || (second.0 >= first.0 && second.1 <= first.1)
        {
            add = 1
        }

        acc + add
    });

    println!("{sum}");
}
