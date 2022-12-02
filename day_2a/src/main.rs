enum GameResult {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

fn get_result(us: char, opp: char) -> GameResult {
    let mut wins = [['a'; 2]; 3];
    wins[0] = ['A', 'C'];
    wins[1] = ['B', 'A'];
    wins[2] = ['C', 'B'];

    let mut win_play = 'Z';
    for i in 0..wins.len() {
        if wins[i][0] == us {
            win_play = wins[i][1];
        }
    }

    match opp {
        _ if us == opp => GameResult::DRAW,
        _ if win_play == opp => GameResult::WIN,
        _ => GameResult::LOSS,
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let score = input.lines().fold(0, |acc, x| {
        let char_string = x.as_bytes();
        let opp: char = char_string[0] as char;

        // shift to be matching with opponent's move
        let us: char = (char_string[2] - 23u8) as char;

        let move_score = match us {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };

        acc + move_score + get_result(us, opp) as i32
    });

    println!("{score}");
}
