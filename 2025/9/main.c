#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  long long row1, col1, row2, col2, area;
} Rec;

bool valid_rec(Rec *rec, long long **points, size_t line_count) {
  long long min_col = (rec->col1 < rec->col2) ? rec->col1 : rec->col2;
  long long max_col = (rec->col1 > rec->col2) ? rec->col1 : rec->col2;
  long long min_row = (rec->row1 < rec->row2) ? rec->row1 : rec->row2;
  long long max_row = (rec->row1 > rec->row2) ? rec->row1 : rec->row2;

  // check no red point is inside rectangle
  for (size_t i = 0; i < line_count; i++) {
    long long *check = points[i];
    long long ccol = check[0];
    long long crow = check[1];

    bool point_inside_col = (ccol > min_col && ccol < max_col);
    bool point_inside_row = (crow > min_row && crow < max_row);

    if (point_inside_col && point_inside_row) {
      return false;
    }
  }

  // check no lines pass through rectangle
  for (size_t i = 0; i < line_count - 1; i++) {
    // next point is promised to be next in outline
    long long *p1 = points[i];
    long long *p2 = points[i + 1];

    long long ccol1 = p1[0];
    long long crow1 = p1[1];
    long long ccol2 = p2[0];
    long long crow2 = p2[1];

    // horizontal lines
    if (crow1 == crow2) {
      long long r = crow1;
      if (r > min_row && r < max_row) {
        long long seg_min_col = (ccol1 < ccol2) ? ccol1 : ccol2;
        long long seg_max_col = (ccol1 > ccol2) ? ccol1 : ccol2;

        if (seg_max_col > min_col && seg_min_col < max_col) {
          return false;
        }
      }
      // vertical lines
    } else if (ccol1 == ccol2) {
      long long c = ccol1;
      if (c > min_col && c < max_col) {
        long long seg_min_row = (crow1 < crow2) ? crow1 : crow2;
        long long seg_max_row = (crow1 > crow2) ? crow1 : crow2;

        if (seg_max_row > min_row && seg_min_row < max_row) {
          return false;
        }
      }
    }
  }

  return true;
}

bool valid(long long *point, long long **points, size_t line_count) {
  // printf("point: %llu,%llu\n", point[0], point[1]);
  bool right_up = false;
  bool right_down = false;
  bool left_up = false;
  bool left_down = false;
  bool up = false;
  bool down = false;
  bool right = false;
  bool left = false;

  for (size_t i = 0; i < line_count; i++) {
    long long *check = points[i];
    // col(left to right), row(top to bottom)

    // the point itself is a red point
    if ((check[0] == point[0] && check[1] == point[1])) {
      // printf("valid point A\n");
      return true;
    }

    // todo: refactor this
    // on same row
    if (check[1] == point[1]) {
      // check on right
      if (check[0] > point[0]) {
        right = true;
      } else {
        left = true;
      }
      // on same col
    } else if (check[0] == point[0]) {
      // check below
      if (check[1] > point[1]) {
        down = true;
      } else {
        up = true;
      }
      // check above point
    } else if (check[1] < point[1]) {
      // check on right
      if (check[0] > point[0]) {
        right_up = true;
      } else {
        left_up = true;
      }
      // check below point
    } else if (check[1] > point[1]) {
      // check on right
      if (check[0] > point[0]) {
        right_down = true;
      } else {
        left_down = true;
      }
    }
  }

  // printf("point: %llu,%llu\n", point[0], point[1]);
  // printf("up: %d, right: %d, down: %d, left: %d\n", up, right, down, left);
  // printf("right_up: %d, left_up: %d, right_down: %d, left_down: %d\n",
  // right_up, left_up, right_down, left_down);

  if ((up && down) || (left && right) ||
      (right_up && right_down && left_up && left_down)) {
    // printf("valid point B\n");
    return true;
  }

  return false;
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
      if (area > max_area) {
        max_area = area;
      }
    }
  }

  printf("pt one: %llu\n", max_area);

  // ------------------ pt two ---------------
  long long pt_two = 0;

  long long area_count = line_count * line_count;
  Rec **recs = malloc(area_count * sizeof(Rec *));
  size_t area_idx = 0;

  for (size_t i = 0; i < area_count; i++) {
    recs[i] = malloc(sizeof(Rec));
  }

  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = 0; j < line_count; j++) {
      long long *p1 = points[i];
      long long *p2 = points[j];
      // this solution did not account for certain edge cases :(
      // // same points or adjacent corners
      // // do I need to include adjacent corners?
      // if (p1[0] == p2[0] || p1[1] == p2[1]) {
      //   continue;
      // }
      //
      // long long p3[2] = {p1[0], p2[1]};
      // long long p4[2] = {p2[0], p1[1]};
      //
      // // printf("rec: %llu,%llu %llu,%llu %llu,%llu %llu,%llu\n", p1[0],
      // p1[1],
      // //        p2[0], p2[1], *p3[0], *p3[1], *p4[0], *p4[1]);
      //
      // if (valid(p3, points, line_count) && valid(p4, points, line_count)) {
      //   long long area =
      //       llabs((p1[0] - p2[0]) + 1) * llabs((p1[1] - p2[1]) + 1);
      //   if (area > pt_two) {
      //     // printf("rec: %llu,%llu %llu,%llu %llu,%llu %llu,%llu\n", p1[0],
      //     p1[1],
      //     //        p2[0], p2[1], p3[0], p3[1], p4[0], p4[1]);
      //     // printf("valid\n");
      //     // printf("area: %llu\n", area);
      //     pt_two = area;
      //   }
      // }
      long long area = llabs((p1[0] - p2[0]) + 1) * llabs((p1[1] - p2[1]) + 1);

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
    if (valid_rec(recs[i], points, line_count)) {
      pt_two = recs[i]->area;
      break;
    }
  }

  // long long e1[2] = {83178, 84037};
  // long long e2[2] = {9, 1};
  // long long e3[2] = {7, 0};
  // long long e4[2] = {8, 1};
  // printf("e1: %d\n", valid(e1, points, line_count));
  // printf("e2: %d\n", valid(e2, points, line_count));
  // printf("e3: %d\n", valid(e3, points, line_count));
  // printf("e4: %d\n", valid(e4, points, line_count));

  // 4633892172 too high
  // 4633889290 too high
  // not 3781050, no feedback
  // not 1396463530, no feedback
  printf("pt two: %llu\n", pt_two);

  // ---------------- cleanup ---------------
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
