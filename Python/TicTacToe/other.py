def main():
    spots = [" " for _ in range(10)]
    player = "O"

    while is_free_slots(spots) and not is_win(spots):
        player = "O" if player == "X" else "X"
        render(spots, player)
        loc = user_input("Input location 1-9:> ")
        while not is_free(spots, loc):
            loc = user_input("Input location 1-9:> ")
        spots[loc] = player
    render(spots, player)
    if is_win(spots):
        print(f"{player} is the winner!")
    elif not is_free_slots(spots):
        print("Draw this round!")

def is_free_slots(spots: list):
    for s in spots:
        if s == " ": return True
    return False

def is_free(spots: list, loc: int):
    if spots[loc] == " ":
        return True
    return False

def user_input(msg: str):
    result = "n"
    while not result[0].isdigit():
        result = input(msg)
    return int(result[0]) - 1

def render(l: list, player: str):
    print("\033[2J\033[1;1H")
    print(f"{l[0]}|{l[1]}|{l[2]}\n-----\n{l[3]}|{l[4]}|{l[5]}\n-----\n{l[6]}|{l[7]}|{l[8]}")

def is_win(s):
    return \
      s[0] == s[1] == s[2] and s[0] != " " or s[3] == s[4] == s[5] and s[3] != " " or \
      s[6] == s[7] == s[8] and s[6] != " " or s[0] == s[3] == s[6] and s[3] != " " or \
      s[1] == s[4] == s[7] and s[1] != " " or s[2] == s[5] == s[8] and s[2] != " " or \
      s[0] == s[4] == s[8] and s[0] != " " or s[2] == s[4] == s[6] and s[2] != " "

if __name__ == "__main__":
    main()
