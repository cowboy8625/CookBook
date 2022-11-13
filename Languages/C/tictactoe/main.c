#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>
#include <stdbool.h>

struct termios orig_termios;

void disableRawMode() {
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
}

void enableRawMode() {
  tcgetattr(STDIN_FILENO, &orig_termios);
  atexit(disableRawMode);

  struct termios raw = orig_termios;
  raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
  raw.c_oflag &= ~(OPOST);
  raw.c_cflag |= (CS8);
  raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
  raw.c_cc[VMIN] = 0;
  raw.c_cc[VTIME] = 1;

  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
}

typedef enum {
  X = 'X',
  O = 'O',
  Cat = ' '
} Player;

void clear() {
  printf("\033[2J\033[1;1H");
}

void render(char s[]) {
  clear();
  printf("   |   |   \r\n");
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

void place(char* s, int n, char sign) {
  s[n - 1] = sign;
}

bool is_win(Player sign, char* s) {
  int loc[8][3] = {
    {0,1,2},
    {3,4,5},
    {6,7,8},
    {0,4,8},
    {2,4,6},
    {0,3,6},
    {1,4,7},
    {2,5,8},
  };
  int a, b, c;
  for (int i = 0; i < sizeof(loc) / sizeof(loc[0]); i++) {
    a = loc[i][0];
    b = loc[i][1];
    c = loc[i][2];
    if ((s[a] == sign) && (s[b] == sign) && (s[c] == sign)) 
      return true;
  }
  return false;
}

bool is_cat(char* s) {
  int count = 0;
  for (int i = 0; i < sizeof(s); i++) {
    if (s[i] == Cat)
      count += 1;
    if (count == 0)
      return true;
    else
      return false;
  }
}

int main(void) {

  enableRawMode();
  char spots[9] = {' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '};

  while (1) {

    render(spots);

    char c = '\0';
    read(STDIN_FILENO, &c, 1);
    if (iscntrl(c)) {
      printf("%d\r\n", c);
    } else {
      printf("%d ('%c')\r\n", c, c);
    }
    if (c == 'q') break;

    if (is_win(X, spots)) {
      printf("X Win's this round!\n");
      break;
    } else if (is_win(O, spots)) {
      printf("O Win's this round!\n");
      break;
    } else if (is_cat(spots)) {
      printf("Cat Win's WOOOOOOO!\n");
      break;
    }
    switch (c) {
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9':
      case 'a':
        printf("lkjasdkjsdfadflkjsdf\r\n");
        place(spots, c, X);
        break;
    }
  }
  return 0;
}

