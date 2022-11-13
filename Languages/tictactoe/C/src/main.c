#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <limits.h>
#include "include/main.h"

int main(void) {
  char spots[9] = "         ";
  unsigned int loc;
  Player player = O;

  do {
    player = (player == X) ? O : X;
    render(spots, player);

    do {
      loc = input("Input location 1-9:> ");
    } while(!isFree(spots, loc));
    place(spots, loc, player);

  } while (isFreeSlots(spots) && !isWin(spots));

  render(spots, player);
  if (isWin(spots)) {
    printf("%c is the winner!", player);
  } else if (!isFreeSlots(spots)) {
    printf("Draw this round!");
  } else {
    printf("ERROR");
  }
  return 0;
}

bool isFreeSlots(char *spots) {
  for(int i = 0; i<sizeof(spots); i++) {
    if (spots[i] == E) return true;
  }
  return false;
}

bool isFree(char *spots, unsigned int loc) {
  if (spots[loc] == E) {
    printf("loc: '%d' is free! '%c'\n", loc, spots[loc]);
    return true;
  }
  return false;
}

unsigned int input(char *msg) {
  char result[10];
  do {
    printf("%s", msg);
    scanf("%9s", result);
  } while(result[0] < 49 || result[0] > 57);
  return result[0] - 49;
}

void place(char *s, unsigned int n, char sign) {
    s[n] = sign;
}

void clear(void) {
  printf("\033[2J\033[1;1H");
}

void render(char *s, char sign) {
  clear();
  printf("   |   |   Player %c turn\r\n", sign);
  printf(" %c | %c | %c \r\n", s[0], s[1], s[2]);
  printf("   |   |   \r\n");
  printf("-----------\r\n");
  printf("   |   |   \r\n");
  printf(" %c | %c | %c \r\n", s[3], s[4], s[5]);
  printf("   |   |   \r\n");
  printf("-----------\r\n");
  printf("   |   |   \r\n");
  printf(" %c | %c | %c \r\n", s[6], s[7], s[8]);
  printf("   |   |   \r\n");
}

bool isWin(char *s) {
    return
      (s[0] == s[1]) && (s[1] == s[2]) && (s[0] != E) ||
      (s[3] == s[4]) && (s[4] == s[5]) && (s[3] != E) ||
      (s[6] == s[7]) && (s[7] == s[8]) && (s[6] != E) ||
      (s[0] == s[3]) && (s[3] == s[6]) && (s[3] != E) ||
      (s[1] == s[4]) && (s[4] == s[7]) && (s[1] != E) ||
      (s[2] == s[5]) && (s[5] == s[8]) && (s[2] != E) ||
      (s[0] == s[4]) && (s[4] == s[8]) && (s[0] != E) ||
      (s[2] == s[4]) && (s[4] == s[6]) && (s[2] != E);
}
