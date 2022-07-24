package main

import (
    "fmt"
    "strconv"
)

const (
    X string = "X"
    O        = "O"
    E        = " "
)

func main() {
    spots := [9]string{E, E, E, E, E, E, E, E, E}
    loc := 0
    player := O
    for isFreeSlots(spots) && !isWin(spots) {
        swap_player(&player)
        render(spots, player)
        for {
            loc = input("Input location 1-9:> ")
            if isFree(spots, loc) {
                break
            }
        }
        place(&spots, loc, player)
    }
    render(spots, player)
    if isWin(spots) {
        fmt.Printf("%s is the winner!", player);
    } else if (!isFreeSlots(spots)) {
        fmt.Println("Draw this round!");
    }
}

func swap_player(player *string) {
    if *player == "X" {
        *player = "O"
    } else {
        *player = "X"
    }
}

func isFreeSlots(spots [9]string) bool {
    for i := 0; i<len(spots); i++ {
        if spots[i] == E {
            return true
        }
    }
    return false
}

func isFree(spots [9]string, loc int) bool {
  if spots[loc] == E {
    return true;
  }
  return false;
}

func input(msg string) int {
  number := ""
  for {
      fmt.Print(msg)
      fmt.Scan(&number)
      fmt.Println(number)
      i, err := strconv.Atoi(number)
      if err == nil && i > 0 && i < 10 {
          return i - 1
      }
  }
  panic("shouldn't ever get here")
}

func place(s *[9]string, n int, sign string) {
    s[n] = sign;
}

func clear() {
  fmt.Println("\033[2J\033[1;1H")
}

func render(s [9]string, sign string) {
  clear()
  fmt.Printf("   |   |   Player %s turn\r\n", sign)
  fmt.Printf(" %s | %s | %s \r\n", s[0], s[1], s[2])
  fmt.Printf("   |   |   \r\n")
  fmt.Printf("-----------\r\n")
  fmt.Printf("   |   |   \r\n")
  fmt.Printf(" %s | %s | %s \r\n", s[3], s[4], s[5])
  fmt.Printf("   |   |   \r\n")
  fmt.Printf("-----------\r\n")
  fmt.Printf("   |   |   \r\n")
  fmt.Printf(" %s | %s | %s \r\n", s[6], s[7], s[8])
  fmt.Printf("   |   |   \r\n")
}

func isWin(s [9]string) bool {
    return (s[0] == s[1]) && (s[1] == s[2]) && (s[0] != E) ||
      (s[3] == s[4]) && (s[4] == s[5]) && (s[3] != E) ||
      (s[6] == s[7]) && (s[7] == s[8]) && (s[6] != E) ||
      (s[0] == s[3]) && (s[3] == s[6]) && (s[3] != E) ||
      (s[1] == s[4]) && (s[4] == s[7]) && (s[1] != E) ||
      (s[2] == s[5]) && (s[5] == s[8]) && (s[2] != E) ||
      (s[0] == s[4]) && (s[4] == s[8]) && (s[0] != E) ||
      (s[2] == s[4]) && (s[4] == s[6]) && (s[2] != E)
}
