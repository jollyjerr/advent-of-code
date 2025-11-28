#include <stdio.h>
#include <stdlib.h>

int smallest_item(int *items, int n) {
  int min = items[0];

  for (int i = 1; i < n; i++) {
    if (items[i] < min) {
      min = items[i];
    }
  }

  return min;
}

int main(int argc, char *argv[]) {
  char *data_path = argv[1];
  printf("data at %s\n", data_path);

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed");
    return EXIT_FAILURE;
  }

  int total_sqft = 0;
  int length, width, height;
  while (fscanf(file, "%dx%dx%dx", &length, &width, &height) == 3) {
    int sides[3] = {length * width, width * height, height * length};

    int smallest_side = smallest_item(sides, sizeof(sides) / sizeof(sides[0]));

    total_sqft += 2 * sides[0] + 2 * sides[1] + 2 * sides[2] + smallest_side;
  }

  printf("Total sqft: %d\n", total_sqft);

  fclose(file);
  return EXIT_SUCCESS;
}
