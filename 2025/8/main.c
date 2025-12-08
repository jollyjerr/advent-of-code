#include <float.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

float euclidean_distance(int p[3], int q[3]) {
  return sqrt(pow((p[0] - q[0]), 2) + pow(p[1] - q[1], 2) +
              pow(p[2] - q[2], 2));
}

bool find_closest(int **boxes, size_t *group_ids, size_t line_count,
                  size_t *box1_idx, size_t *box2_idx) {
  bool found = false;
  float min_distance = FLT_MAX;

  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = i + 1; j < line_count; j++) {
      float dist = euclidean_distance(boxes[i], boxes[j]);
      if (dist < min_distance && (group_ids[i] == 0 || group_ids[j] == 0 ||
                                  group_ids[i] != group_ids[j])) {
        found = true;
        min_distance = dist;
        *box1_idx = i;
        *box2_idx = j;
      }
    }
  }

  return found;
}

int main(int argc, char *argv[]) {
  FILE *file = fopen(argv[1], "r");

  size_t line_count = 0;
  char line[256];
  while (fgets(line, sizeof(line), file) != NULL) {
    line_count++;
  }

  rewind(file);

  int **boxes = malloc(line_count * sizeof(int *));
  for (size_t i = 0; i < line_count; i++) {
    boxes[i] = malloc(3 * sizeof(int));
  }

  size_t box_idx = 0;
  int X, Y, Z;
  while (fscanf(file, "%d,%d,%d", &X, &Y, &Z) == 3) {
    boxes[box_idx][0] = X;
    boxes[box_idx][1] = Y;
    boxes[box_idx][2] = Z;
    box_idx++;
  }

  fclose(file);

  // pt one
  size_t group_id = 0;
  size_t *group_ids = malloc(line_count * sizeof(size_t));
  for (int i = 0; i < line_count; i++) {
    group_ids[i] = 0;
  }

  size_t box1;
  size_t box2;
  int iterations = 0;
  while (find_closest(boxes, group_ids, line_count, &box1, &box2) == true) {
    if (iterations == 10) {
      break;
    }
    // printf("found: %zu, %zu\n", box1, box2);

    size_t gid1 = group_ids[box1];
    size_t gid2 = group_ids[box2];

    // printf("what: %zu, %zu\n", gid1, gid2);

    if (gid1 == 0 && gid2 == 0) {
      group_id++;

      group_ids[box1] = group_id;
      group_ids[box2] = group_id;
    } else if (gid1 == 0) {
      group_ids[box1] = gid2;
    } else if (gid2 == 0) {
      group_ids[box2] = gid1;
    } else {
      // union 1 consumes 2
      for (size_t i = 0; i < line_count; i++) {
        if (group_ids[i] == gid2) {
          group_ids[i] = gid1;
        }
      }
    }

    iterations++;
  }

  for (size_t i = 0; i < line_count; i++) {
    printf("%zu\n", group_ids[i]);
  }

  for (size_t i = 0; i < line_count; i++) {
    free(boxes[i]);
  }
  free(boxes);
  free(group_ids);
  return EXIT_SUCCESS;
}
