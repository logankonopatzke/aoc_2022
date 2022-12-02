enum GameResult {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

fn get_move_score(play: char) -> i32 {
    match play {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn get_total_score(us: char, opp: char) -> i32 {
    let mut wins = [['a'; 2]; 3];
    wins[0] = ['A', 'C'];
    wins[1] = ['B', 'A'];
    wins[2] = ['C', 'B'];

    if us == 'B' {
        return GameResult::DRAW as i32 + get_move_score(opp);
    }

    for i in 0..wins.len() {
        if us == 'A' && wins[i][0] == opp
        // loss
        {
            return GameResult::LOSS as i32 + get_move_score(wins[i][1]);
        }

        if us == 'C' && wins[i][1] == opp
        // win
        {
            return GameResult::WIN as i32 + get_move_score(wins[i][0]);
        }
    }

    0
}

fn main() {
    let input = include_str!("../input.txt");

    let score = input.lines().fold(0, |acc, x| {
        let char_string = x.as_bytes();
        let opp: char = char_string[0] as char;

        // shift to be matching with opponent's move
        let us: char = (char_string[2] - 23u8) as char;

        acc + get_total_score(us, opp) as i32
    });

    println!("{score}");
}
