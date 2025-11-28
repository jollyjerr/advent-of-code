#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int value;
  int index;
} MinResult;

MinResult min(int *items, int n) {
  int index = 0;
  int value = items[index];

  for (int i = 1; i < n; i++) {
    if (items[i] < value) {
      value = items[i];
      index = i;
    }
  }

  MinResult out = {value, index};
  return out;
}

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed");
    return EXIT_FAILURE;
  }

  int total_sqft = 0;
  int total_ribbon = 0;

  int length, width, height;
  while (fscanf(file, "%dx%dx%dx", &length, &width, &height) == 3) {
    // pt 1
    int side_areas[3] = {length * width, width * height, height * length};

    MinResult smallest_side = min(side_areas, 3);
    int wrapping = 2 * side_areas[0] + 2 * side_areas[1] + 2 * side_areas[2] +
                   smallest_side.value;
    total_sqft += wrapping;

    // pt 2
    int n = 3;
    int dims[3] = {length, width, height};
    MinResult smallest_dim = min(dims, 3);

    // remove smallest element with swap
    int temp = dims[smallest_dim.index];
    dims[smallest_dim.index] = dims[n - 1];
    dims[n - 1] = temp;
    n--;

    MinResult next_smallest_dim = min(dims, n);
    int ribbon = (length * width * height) + (2 * smallest_dim.value) +
                 (2 * next_smallest_dim.value);
    total_ribbon += ribbon;
  }

  printf("Total sqft: %d\n", total_sqft);
  printf("Total ribbon: %d\n", total_ribbon);

  fclose(file);
  return EXIT_SUCCESS;
}
