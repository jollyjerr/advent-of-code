#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed\n");
    return EXIT_FAILURE;
  }

  int dial_value = 50;

  int pt_one = 0;
  int pt_two = 0;

  char direction;
  int count;
  while (fscanf(file, " %c%d", &direction, &count) == 2) {
    int step = (direction == 'R') ? 1 : -1;

    for (int i = 0; i < count; i++) {
      dial_value += step;

      if (dial_value == 100) {
        dial_value = 0;
      } else if (dial_value == -1) {
        dial_value = 99;
      }

      if (dial_value == 0) {
        pt_two++;
      }
    }

    if (dial_value == 0) {
      pt_one++;
    }
  }

  printf("pt one: %d\n", pt_one);
  printf("pt two: %d\n", pt_two);

  fclose(file);

  return EXIT_SUCCESS;
}
