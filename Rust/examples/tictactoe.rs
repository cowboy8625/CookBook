use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut board: Board = [' '; 9];
    let mut sign = 'X';
    loop {
        clear();
        render(board);
        let num = input("Pick a number 1 - 9:> ");
        if is_empty(board, num) {
            set_spot(&mut board, num, sign);
            sign = if sign == 'X' { 'O' } else { 'X' };
        }
        let (win, winner) = check_board(board);
        if win {
            clear();
            render(board);
            match winner {
                'X' => break println!("Looks like X own this one!"),
                'O' => break println!("O is the winner!!"),
                'C' => break println!("Cat got this one!"),
                ' ' => {}
                _ => break println!("Something went wrong."),
            }
        }
    }
}

type Board = [char; 9];

fn check_board(board: Board) -> (bool, char) {
    let win = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 4, 8),
        (2, 4, 6),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
    ];
    let mut count = 0;
    for &(a, b, c) in win.iter() {
        if board[a] == board[b] && board[a] == board[c] {
            return (true, board[a]);
        }
    }
    for &ch in board.iter() {
        if ch == ' ' {
            count += 1;
        }
    }
    if count == 0 {
        return (true, 'C');
    }
    (false, ' ')
}

fn is_empty(board: Board, num: usize) -> bool {
    if board[num] == ' ' {
        true
    } else {
        false
    }
}

fn set_spot(spots: &mut Board, i: usize, sign: char) {
    spots[i] = sign
}

fn clear() {
    match std::process::Command::new("clear").status() {
        Ok(_) => {}
        Err(_) => match std::process::Command::new("cls").status() {
            Ok(_) => {}
            Err(_) => {}
        },
    }
}

fn input(message: &str) -> usize {
    let mut out = String::new();
    loop {
        print!("{}", message);
        stdout().flush().unwrap();
        stdin().read_line(&mut out).expect("Failed to read line");
        let num = out.trim().parse::<usize>();
        match num {
            Ok(n) => match n {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => return n - 1,
                _ => println!("The number needs to be between 1 - 9."),
            },
            Err(e) => println!("{}", e),
        }
        out.clear();
    }
}

fn render(spots: Board) {
    println!(" {} | {} | {} ", spots[0], spots[1], spots[2]);
    println!("-----------");
    println!(" {} | {} | {} ", spots[3], spots[4], spots[5]);
    println!("-----------");
    println!(" {} | {} | {} ", spots[6], spots[7], spots[8]);
}
