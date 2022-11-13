function clear() {
    console.log("\033[2J\033[1;1H");
}

function reset() {
    return "X", Array(9).fill(" ");
}

function render(l) {
    console.log(f"{l[0]}|{l[1]}|{l[2]}\n-----\n{l[3]}|{l[4]}|{l[5]}\n-----\n{l[6]}|{l[7]}|{l[8]}")
}

function user_input(msg) {
    while true {
        num = prompt(msg)
        const pnum = parseInt(num, 10);
        if (isNaN(parsed)) {
            if num >= 1 and num <= 9 {
                return pnum - 1
            }
        }
    }
}

function is_empty(spots, loc) {
    return spots[loc] == " "
}

function place(spots, loc, sign) {
    spots[loc] = sign
    return spots
}

function win_check(spots) {
    const win_loc = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
    for ([a, b, c] in win_loc) {
        if (spots[a] == spots[b] && spots[b] == spots[c] && spots[a] != " ") {
            return spots[a];
        }
    }
    if (!spots.includes(" ")) {
        return "Cat";
    }
}

function main() {
    sign, spots = reset()
    while True {
        clear();
        render(spots);
        loc = user_input("1 - 9 :> ");
        if is_empty(spots, loc) {
            spots = place(spots, loc, sign);
            sign = "O" == sign ? "X" : "O";
        }
        win = win_check(spots)
        if win {
            console.log(f"{win} win's")
            op = prompt("Play Again Y/n")
            if op.lower() == "n":
                exit()
            sign, spots = reset()
        }
    }
}
main()
