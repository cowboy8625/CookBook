import os

def reset():
    return "X", [" " for i in range(9)]

def render(l: list):
    print(f"{l[0]}|{l[1]}|{l[2]}\n-----\n{l[3]}|{l[4]}|{l[5]}\n-----\n{l[6]}|{l[7]}|{l[8]}")

def user_input(msg: str):
    while True:
        num = input(msg)
        if num.isdigit():
            num = int(num)
            if num >= 1 and num <= 9:
                return num - 1

def is_empty(spots: list, loc: int):
    return True if spots[loc] == " " else False

def place(spots: list, loc: int, sign: str):
    spots[loc] = sign
    return spots

def win_check(spots: list):
    win_loc = ((0,1,2),(3,4,5),(6,7,8),(0,3,6),(1,4,7),(2,5,8),(0,4,8),(2,4,6))
    for a, b, c in win_loc:
        if spots[a] == spots[b] == spots[c] != " ":
            return spots[a]
    if " " not in spots:
        return "Cat"

def main():
    sign, spots = reset()
    while True:
        os.system("cls" if os.name == "nt" else "clear")
        render(spots)
        loc = user_input("1 - 9 :> ")
        if is_empty(spots, loc):
            spots = place(spots, loc, sign)
            sign = "X" if sign == "O" else "O"
        win = win_check(spots) 
        if win:
            print(f"{win} win's")
            op = input("Play Again Y/n")
            if op.lower() == "n":
                exit()
            sign, spots = reset()

if __name__ == "__main__":
    main()
