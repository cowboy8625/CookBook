#ifndef MAIN_H
#define MAIN_H

typedef enum {
  X = 'X',
  O = 'O',
  E = ' '
} Player;

bool isFreeSlots(char *spots);
bool isFree(char *spots, unsigned int loc);
unsigned int input(char *msg);
void clear(void);
void render(char *s, char sign);
void place(char *s, unsigned int n, char sign);
bool isWin(char *s);

#endif
