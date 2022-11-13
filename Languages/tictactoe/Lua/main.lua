Player = { X="X", O="O", E=" " }

function main()
    local spots = {
        Player.E,
        Player.E,
        Player.E,
        Player.E,
        Player.E,
        Player.E,
        Player.E,
        Player.E,
        Player.E,
    }
    local loc = 0
    local player = Player.O
    repeat
        player = swap_player(player)
        render(spots, player)
        repeat
            loc = input("Input location 1-9:> ")
        until(isFree(spots, loc))
        place(spots, loc, player);
    until(not isFreeSlots(spots) and isWin(spots))
    render(spots, player)
    if isWin(spots) then
        print(string.format("%s is the winner!", player))
    elseif not isFreeSlots(spots) then
        print("Draw this round!")
    end
end

function swap_player(player)
    if player == "X" then
        return "O"
    else
        return "X"
    end
end

function isFreeSlots(spots)
    for i = 1,#spots do
        if spots[i] == E then
            return true
        end
    end
    return false
end

function isFree(spots, loc)
    if spots[loc] == Player.E then
        return true
    end
    return false
end

function input(msg)
    repeat
        io.write(msg)
        loc = io.read("*n")
    until(loc ~= nil and loc > 0 and loc < 10)
    return loc
end

function place(s, n, sign)
    s[n] = sign
end

function clear()
  print("\x1b[2J\x1b[1;1H")
end

function render(s, sign)
  clear()
  print(string.format("   |   |   Player %s turn", sign))
  print(string.format(" %s | %s | %s ", s[1], s[2], s[3]))
  print(string.format("   |   |   "))
  print(string.format("-----------"))
  print(string.format("   |   |   "))
  print(string.format(" %s | %s | %s ", s[4], s[5], s[6]))
  print(string.format("   |   |   "))
  print(string.format("-----------"))
  print(string.format("   |   |   "))
  print(string.format(" %s | %s | %s ", s[7], s[8], s[9]))
  print(string.format("   |   |   "))
end

function isWin(s)
    return
      (s[1] == s[2]) and (s[2] == s[3]) and (s[1] ~= Player.E) or
      (s[4] == s[5]) and (s[5] == s[6]) and (s[4] ~= Player.E) or
      (s[7] == s[8]) and (s[8] == s[9]) and (s[7] ~= Player.E) or
      (s[1] == s[4]) and (s[4] == s[7]) and (s[4] ~= Player.E) or
      (s[2] == s[5]) and (s[5] == s[8]) and (s[2] ~= Player.E) or
      (s[3] == s[6]) and (s[6] == s[9]) and (s[3] ~= Player.E) or
      (s[1] == s[5]) and (s[5] == s[9]) and (s[1] ~= Player.E) or
      (s[3] == s[5]) and (s[5] == s[7]) and (s[3] ~= Player.E);
end

main()
