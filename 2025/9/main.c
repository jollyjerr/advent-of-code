#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  FILE *file = fopen(argv[1], "r");

  size_t line_count = 0;
  char line[256];
  while (fgets(line, sizeof(line), file) != NULL) {
    line_count++;
  }

  rewind(file);

  long long **points = malloc(line_count * sizeof(long long *));
  for (size_t i = 0; i < line_count; i++) {
    points[i] = malloc(2 * sizeof(long long));
  }

  size_t point_id = 0;
  long long y, x;
  while (fscanf(file, "%llu,%llu", &x, &y) == 2) {
    points[point_id][0] = x;
    points[point_id][1] = y;
    point_id++;
  }

  fclose(file);

  // ---------------- pt one --------------------
  long long max_area = 0;
  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = 0; j < line_count; j++) {
      long long area = llabs((points[i][0] - points[j][0]) + 1) *
                       llabs((points[i][1] - points[j][1]) + 1);
      // printf("A: %llu,%llu\n", points[i][0], points[i][1]);
      // printf("B: %llu,%llu\n", points[j][0], points[j][1]);
      // printf("Area: %llu\n", area);
      if (area > max_area) {
        max_area = area;
      }
    }
  }

  printf("pt one: %llu\n", max_area);

  for (size_t i = 0; i < line_count; i++) {
    free(points[i]);
  }
  free(points);
  return EXIT_SUCCESS;
}
