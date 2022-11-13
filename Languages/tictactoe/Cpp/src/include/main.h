#ifndef MAIN_H
#define MAIN_H
typedef enum {
  X = 'X',
  O = 'O',
  E = ' '
} Player;

bool isFreeSlots(Player spots[]);
bool isFree(Player spots[], unsigned int loc);
unsigned int input(const char *msg);
void clear(void);
void render(Player spots[], Player sign);
void place(Player spots[], unsigned int n, Player sign);
bool isWin(Player s[]);

#endif
