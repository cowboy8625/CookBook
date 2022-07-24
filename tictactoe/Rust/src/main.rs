use std::fmt::{self, Display};
use std::io::{stdin, stdout, Write};

type Board = [Sign; 9];

fn main() {
    let mut board: Board = [Sign::E; 9];
    let mut sign = Sign::O;
    let mut loc;
    while !is_win(&board) || is_board_free(&board) {
        sign = if sign == Sign::X { Sign::O } else { Sign::X };
        render(&board, &sign);
        loc = input("Input 1-9 :> ");
        while !is_free(&board, loc) {
            loc = input("Input 1-9 :> ");
        }
        board[loc] = sign;
    }
    render(&board, &sign);
    if is_win(&board) {
        println!("{} is the winner!", sign);
    } else if !is_board_free(&board) {
        println!("Draw this round!");
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Sign {
    X,
    O,
    E,
}

impl Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = match self {
            Self::X => 'X',
            Self::O => 'O',
            Self::E => ' ',
        };
        write!(f, "{}", sign)
    }
}

fn is_win(b: &Board) -> bool {
    (b[0] == b[1]) && (b[1] == b[2]) && (b[0] != Sign::E)
        || (b[3] == b[4]) && (b[4] == b[5]) && (b[3] != Sign::E)
        || (b[6] == b[7]) && (b[7] == b[8]) && (b[6] != Sign::E)
        || (b[0] == b[3]) && (b[3] == b[6]) && (b[3] != Sign::E)
        || (b[1] == b[4]) && (b[4] == b[7]) && (b[1] != Sign::E)
        || (b[2] == b[5]) && (b[5] == b[8]) && (b[2] != Sign::E)
        || (b[0] == b[4]) && (b[4] == b[8]) && (b[0] != Sign::E)
        || (b[2] == b[4]) && (b[4] == b[6]) && (b[2] != Sign::E)
}

fn is_board_free(board: &Board) -> bool {
    board.iter().fold(false, |acc, c| acc || c == &Sign::E)
}

fn is_free(board: &Board, loc: usize) -> bool {
    board[loc] == Sign::E
}

fn clear() {
    println!("\x1b[2J\x1b[0;0H");
}

fn input(message: &str) -> usize {
    let mut out = String::new();
    while out.chars().nth(0).map(|i| !i.is_numeric()).unwrap_or(true) {
        print!("{}", message);
        stdout().flush().expect("Failed to flush to stdout.");
        stdin().read_line(&mut out).expect("Failed to read line");
    }
    out[0..1].parse::<usize>().unwrap() - 1
}

fn render(spots: &Board, sign: &Sign) {
    clear();
    println!("                    The board is numbered");
    println!(
        " {} │ {} │ {}  [{}: Turn]    1 │ 2 │ 3",
        spots[0], spots[1], spots[2], sign
    );
    println!("⎼⎼⎼╄⎼⎼⎼╄⎼⎼⎼             ⎼⎼⎼╄⎼⎼⎼╄⎼⎼⎼");
    println!(
        " {} │ {} │ {}               4 │ 5 │ 6 ",
        spots[3], spots[4], spots[5]
    );
    println!("⎼⎼⎼╄⎼⎼⎼╄⎼⎼⎼             ⎼⎼⎼╄⎼⎼⎼╄⎼⎼⎼");
    println!(
        " {} │ {} │ {}               7 │ 8 │ 9 ",
        spots[6], spots[7], spots[8]
    );
}
