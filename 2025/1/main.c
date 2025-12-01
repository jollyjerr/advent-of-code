#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed");
    return EXIT_FAILURE;
  }

  char direction;
  int count;

  int value = 50;
  int out = 0;

  while (fscanf(file, " %c%d", &direction, &count) == 2) {
    if (direction == 'R') {
      for (int i = 0; i < count; i++) {
        value += 1;

        if (value == 100) {
          value = 0;
        }
      }
    } else {
      for (int i = 0; i < count; i++) {
        value -= 1;

        if (value == -1) {
          value = 99;
        }
      }
    }

    if (value == 0) {
      out += 1;
    }
  }

  printf("pt one: %d\n", out);

  fclose(file);

  return EXIT_SUCCESS;
}
