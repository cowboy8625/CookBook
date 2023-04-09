function clear() {
    console.log("\033[2J\033[1;1H");
}
function reset() {
    return ["X", Array(9).fill(" ")];
}
function render(l) {
    console.log("".concat(l[0], "|").concat(l[1], "|").concat(l[2], "\n-----\n").concat(l[3], "|").concat(l[4], "|").concat(l[5], "\n-----\n").concat(l[6], "|").concat(l[7], "|").concat(l[8]));
}
function user_input(msg) {
    while (true) {
        num = prompt(msg);
        var pnum = parseInt(num, 10);
        if (isNaN(parsed)) {
            if (num >= 1 && num <= 9) {
                return pnum - 1;
            }
        }
    }
}
function is_empty(spots, loc) {
    return spots[loc] == " ";
}
function place(spots, loc, sign) {
    spots[loc] = sign;
    return spots;
}
function win_check(spots) {
    var win_loc = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];
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
    var sign, spots = reset();
    while (true) {
        clear();
        render(spots);
        loc = user_input("1 - 9 :> ");
        if (is_empty(spots, loc)) {
            spots = place(spots, loc, sign);
            sign = "O" == sign ? "X" : "O";
        }
        win = win_check(spots);
        if (win) {
            console.log("".concat(win, " win's"));
            op = prompt("Play Again Y/n");
            if (op.lower() == "n") {
                exit();
            }
            sign, spots = reset();
        }
    }
}
main();
