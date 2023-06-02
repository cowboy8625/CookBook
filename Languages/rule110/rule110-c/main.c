#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <raylib.h>

#define WIDTH 200
#define HEIGHT 100
#define SIZE 10
#define SCREEN_WIDTH  WIDTH*SIZE
#define SCREEN_HEIGHT HEIGHT*SIZE

bool is_alive(bool a, bool b, bool c) {
  int input = (a << 2) | (b << 1) | c;
  bool outputs[] = {false, true, true, true, // 0, 1, 2, 3
                    false, true, true, false}; // 4, 5, 6, 7
  return outputs[input];
}
/*

000 -> 0
001 -> 1
010 -> 1

011 -> 1
100 -> 0
101 -> 1
110 -> 1
111 -> 0
--------
000 -> 0

010 -> 1
011 -> 1

101 -> 1
100 -> 0


*/

bool* next_gen(bool (*last_gen)) {
  bool next[WIDTH];
  for(int i = 0; i<WIDTH; ++i) {
    bool a = last_gen[((i - 1) + WIDTH) % WIDTH];
    bool b = last_gen[i];
    bool c = last_gen[((i + 1) + WIDTH) % WIDTH];
    next[i] = is_alive(a, b, c);
  }
  memcpy(last_gen, &next, WIDTH);
}

void update_world(bool (*world)[WIDTH], bool *grid) {
  memmove(&world[0], &world[1], (HEIGHT-1)*WIDTH);
  memset(&world[HEIGHT-1], 0, 1 * WIDTH);
  memcpy(&world[HEIGHT-1], grid, WIDTH);
}

int main() {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "test");
  SetTargetFPS(60);

  bool grid[WIDTH] = {0};
  grid[WIDTH-1]=true;

  bool world[HEIGHT][WIDTH] = {0};
  int loop_count = 0;

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    next_gen(grid);
    update_world(world, grid);

    for(int gy = 0; gy < HEIGHT; gy++) {
      for(int gx = 0; gx < WIDTH; gx++) {
        if (world[gy][gx]) {
          DrawRectangle(gx*SIZE, gy*SIZE, SIZE, SIZE, RED);
        }
      }
    }

    EndDrawing();
  }

  CloseWindow();

  return 0;
}
