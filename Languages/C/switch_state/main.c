#include <stdio.h>

int main(void) {
  int thingToSwitchOver = 5;
  switch(thingToSwitchOver) {
    case 0:
      printf("This is Zero\n");
      break;
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
      printf("Between 1 and 5\n");
      break;
    default:
      printf("Big number\n");
      break;


  }
  printf("Hello World\n");
  return 0;
}
