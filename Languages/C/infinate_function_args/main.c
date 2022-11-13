#include <stdio.h>
#include <string.h>
#include <stdarg.h>

void print(char s[], ...) {
  va_list valist;

  /* initialize valist for num number of arguments */
  va_start(valist, s);

  /* access all the arguments assigned to valist */
  for (int i = 0; i < sizeof(valist); i++) {
     printf("%s", va_arg(valist, char*));
  }
  printf("\n");

  /* clean memory reserved for valist */
  va_end(valist);
}

int main() {
  print("Hey");
  return 0;
}
