#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  long long x_min, x_max, y_min, y_max;
} Rectangle;

long long min(long long a, long long b) { return a < b ? a : b; }
long long max(long long a, long long b) { return a > b ? a : b; }

Rectangle new_rectangle(long long x1, long long y1, long long x2,
                        long long y2) {
  Rectangle r;
  r.x_min = min(x1, x2);
  r.x_max = max(x1, x2);
  r.y_min = min(y1, y2);
  r.y_max = max(y1, y2);
  return r;
}

bool overlap(Rectangle *a, Rectangle *b) {
  return !(a->x_max <= b->x_min || b->x_max <= a->x_min ||
           a->y_max <= b->y_min || b->y_max <= a->y_min);
}

long long calc_area(Rectangle *r) {
  long long x_len = r->x_max - r->x_min + 1;
  long long y_len = r->y_max - r->y_min + 1;
  return x_len * y_len;
}

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
  long long x, y;
  while (fscanf(file, "%lld,%lld", &x, &y) == 2) {
    points[point_id][0] = x;
    points[point_id][1] = y;
    point_id++;
  }
  fclose(file);

  // ---------------- pt one --------------------
  long long pt_one = 0;
  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = 0; j < line_count; j++) {
      Rectangle rect =
          new_rectangle(points[i][0], points[i][1], points[j][0], points[j][1]);
      long long area = calc_area(&rect);
      if (area > pt_one) {
        pt_one = area;
      }
    }
  }

  printf("pt one: %lld\n", pt_one);

  // ---------------- pt two --------------------
  Rectangle *edges = malloc(line_count * sizeof(Rectangle));
  for (size_t i = 0; i < line_count - 1; i++) {
    edges[i] = new_rectangle(points[i][0], points[i][1], points[i + 1][0],
                             points[i + 1][1]);
  }

  // last edge from end to start
  edges[line_count - 1] =
      new_rectangle(points[line_count - 1][0], points[line_count - 1][1],
                    points[0][0], points[0][1]);

  long long pt_two = 0;
  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = i + 1; j < line_count; j++) {
      Rectangle rect =
          new_rectangle(points[i][0], points[i][1], points[j][0], points[j][1]);

      bool overlaps = false;
      for (size_t k = 0; k < line_count; k++) {
        if (overlap(&rect, &edges[k])) {
          overlaps = true;
          break;
        }
      }

      if (!overlaps) {
        long long area = calc_area(&rect);
        if (area > pt_two) {
          pt_two = area;
        }
      }
    }
  }

  printf("pt two: %lld\n", pt_two);

  // ------------------- cleanup --------------
  free(edges);
  for (size_t i = 0; i < line_count; i++) {
    free(points[i]);
  }
  free(points);

  return EXIT_SUCCESS;
}
