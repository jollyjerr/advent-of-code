#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  long long row1, col1, row2, col2, area;
} Rec;

long long cal_area(long long x1, long long y1, long long x2, long long y2) {
  return llabs(x1 - x2 + 1) * llabs(y1 - y2 + 1);
}

int compare_rec_desc(const void *a, const void *b) {
  Rec *val_a = *(Rec **)a;
  Rec *val_b = *(Rec **)b;

  if (val_a->area < val_b->area)
    return 1;
  if (val_a->area > val_b->area)
    return -1;
  return 0;
}

void add_side(long long x1, long long y1, long long x2, long long y2,
              long long **side_points, size_t *side_point_idx_ptr) {
  long long min_x = x1 > x2 ? x2 : x1;
  long long max_x = x1 > x2 ? x1 : x2;
  long long min_y = y1 > y2 ? y2 : y1;
  long long max_y = y1 > y2 ? y1 : y2;

  size_t side_point_idx = *side_point_idx_ptr;

  side_points[side_point_idx][0] = x1;
  side_points[side_point_idx][1] = y1;
  side_point_idx++;

  if (x1 == x2) {
    for (long long k = min_y + 1; k < max_y; k++) {
      side_points[side_point_idx][0] = x1;
      side_points[side_point_idx][1] = k;
      side_point_idx++;
    }
  } else if (y1 == y2) {
    for (long long k = min_x + 1; k < max_x; k++) {
      side_points[side_point_idx][0] = k;
      side_points[side_point_idx][1] = y1;
      side_point_idx++;
    }
  }

  *side_point_idx_ptr = side_point_idx;
}

bool valid_point(long long x1, long long y1, long long **side_points,
                 const size_t side_point_idx) {
  // point is on the wall
  for (size_t i = 0; i < side_point_idx; i++) {
    if (side_points[i][0] == x1 && side_points[i][1] == y1) {
      return true;
    }
  }

  // count wall points directly to the right
  int crossings = 0;
  for (size_t i = 0; i < side_point_idx; i++) {
    long long x2 = side_points[i][0];
    long long y2 = side_points[i][1];

    if (y2 == y1 && x2 > x1) {
      crossings++;
    }
  }

  return (crossings % 2) == 1;
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
      long long area =
          cal_area(points[i][0], points[i][1], points[j][0], points[j][1]);
      if (area > max_area) {
        max_area = area;
      }
    }
  }

  printf("pt one: %llu\n", max_area);

  // ------------------ pt two ---------------
  long long pt_two = 0;

  size_t num_side_points = 1000000;
  long long **side_points = malloc(num_side_points * sizeof(long long *));
  for (size_t i = 0; i < num_side_points; i++) {
    side_points[i] = malloc(2 * sizeof(long long));
  }

  // calculate all side points of the wall
  size_t side_point_idx = 0;
  for (size_t i = 0; i < line_count - 1; i++) {
    long long x1 = points[i][0];
    long long y1 = points[i][1];
    long long x2 = points[i + 1][0];
    long long y2 = points[i + 1][1];

    add_side(x1, y1, x2, y2, side_points, &side_point_idx);
  }

  // from last point back to the first
  long long x1_last = points[line_count - 1][0];
  long long y1_last = points[line_count - 1][1];
  long long x2_first = points[0][0];
  long long y2_first = points[0][1];
  add_side(x1_last, y1_last, x2_first, y2_first, side_points, &side_point_idx);

  // calculate and sort recs by area so we can approach largest from above
  long long area_count = line_count * line_count;
  Rec **recs = malloc(area_count * sizeof(Rec *));
  for (size_t i = 0; i < area_count; i++) {
    recs[i] = malloc(sizeof(Rec));
  }

  size_t area_idx = 0;
  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = 0; j < line_count; j++) {
      long long *p1 = points[i], *p2 = points[j];
      long long area = cal_area(p1[0], p1[1], p2[0], p2[1]);

      recs[area_idx]->col1 = p1[0];
      recs[area_idx]->row1 = p1[1];
      recs[area_idx]->col2 = p2[0];
      recs[area_idx]->row2 = p2[1];
      recs[area_idx]->area = area;

      area_idx++;
    }
  }

  qsort(recs, area_count, sizeof(Rec *), compare_rec_desc);

  for (size_t i = 0; i < area_count; i++) {
    long long raw_x1 = recs[i]->col1, raw_y1 = recs[i]->row1;
    long long raw_x2 = recs[i]->col2, raw_y2 = recs[i]->row2;
    long long min_x = (raw_x1 < raw_x2) ? raw_x1 : raw_x2;
    long long max_x = (raw_x1 > raw_x2) ? raw_x1 : raw_x2;
    long long min_y = (raw_y1 < raw_y2) ? raw_y1 : raw_y2;
    long long max_y = (raw_y1 > raw_y2) ? raw_y1 : raw_y2;

    bool perimeter_valid = true;

    for (long long x = min_x; x <= max_x; x++) {
      if (!valid_point(x, min_y, side_points, side_point_idx) ||
          !valid_point(x, max_y, side_points, side_point_idx)) {
        perimeter_valid = false;
        break;
      }
    }

    if (perimeter_valid) {
      for (long long y = min_y + 1; y < max_y; y++) {
        if (!valid_point(min_x, y, side_points, side_point_idx) ||
            !valid_point(max_x, y, side_points, side_point_idx)) {
          perimeter_valid = false;
          break;
        }
      }
    }

    if (perimeter_valid) {
      pt_two = recs[i]->area;
      break;
    }
  }

  // 4633892172 too high
  // 4633889290 too high
  // not 3781050, no feedback
  // not 1396463530, no feedback -- I keep getting this result ahhhhhhh
  // not 1396313832, no feedback
  printf("pt two: %llu\n", pt_two);

  // ---------------- cleanup ---------------
  for (size_t i = 0; i < num_side_points; i++) {
    free(side_points[i]);
  }
  free(side_points);
  for (size_t i = 0; i < area_count; i++) {
    free(recs[i]);
  }
  free(recs);
  for (size_t i = 0; i < line_count; i++) {
    free(points[i]);
  }
  free(points);

  return EXIT_SUCCESS;
}
