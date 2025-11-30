#include <stdio.h>

int main() {
  FILE *file = fopen("data/1.1.txt", "r");
  if (file == NULL) {
    return 1;
  }

  int floor = 0;
  int idx = 1;
  int basement = -1;

  int ch;
  while ((ch = fgetc(file)) != EOF) {
    if (ch == '(') {
      floor++;
    } else if (ch == ')') {
      floor--;
    }

    if (basement < 0 && floor < 0) {
      basement = idx;
    }
    idx++;
  }

  fclose(file);

  printf("floor: %d\n", floor);
  printf("basement: %d\n", basement);

  return 0;
}
