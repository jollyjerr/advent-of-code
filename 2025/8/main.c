#include <float.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int *box1, *box2;
  float distance;
} Connection;

float euclidean_distance(int p[3], int q[3]) {
  return sqrt(pow((p[0] - q[0]), 2) + pow(p[1] - q[1], 2) +
              pow(p[2] - q[2], 2));
}

int compare_connections(const void *a, const void *b) {
  Connection *conn_a = *(Connection **)a;
  Connection *conn_b = *(Connection **)b;

  if (conn_a->distance < conn_b->distance)
    return -1;
  if (conn_a->distance > conn_b->distance)
    return 1;
  return 0;
}

int compare_sizet_desc(const void *a, const void *b) {
  size_t val_a = *(const size_t *)a;
  size_t val_b = *(const size_t *)b;

  if (val_a < val_b)
    return 1;
  if (val_a > val_b)
    return -1;
  return 0;
}

// todo: use the connections list rather than recalculating distances
bool find_next_union_connection(int **boxes, size_t line_count,
                                size_t *box1_idx, size_t *box2_idx) {
  bool found = false;

  float min_distance = FLT_MAX;
  for (size_t i = 0; i < line_count; i++) {
    for (size_t j = i + 1; j < line_count; j++) {
      float dist = euclidean_distance(boxes[i], boxes[j]);
      if (dist < min_distance && (boxes[i][3] == 0 || boxes[j][3] == 0 ||
                                  boxes[i][3] != boxes[j][3])) {
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
    boxes[i] = malloc(4 * sizeof(int));
  }

  size_t box_idx = 0;
  int X, Y, Z;
  while (fscanf(file, "%d,%d,%d", &X, &Y, &Z) == 3) {
    boxes[box_idx][0] = X;
    boxes[box_idx][1] = Y;
    boxes[box_idx][2] = Z;

    // group_id - 0 is ungrouped
    boxes[box_idx][3] = 0;

    box_idx++;
  }

  fclose(file);

  // ------------------- pt one ---------------

  size_t combinations = (line_count * (line_count - 1)) / 2;
  Connection **connections = malloc(combinations * sizeof(Connection *));
  size_t connections_idx = 0;

  size_t head = 1;
  size_t tail = 0;
  while (tail < line_count - 1) {
    float distance = euclidean_distance(boxes[tail], boxes[head]);

    Connection *new_connection = malloc(sizeof(Connection));

    new_connection->box1 = boxes[tail];
    new_connection->box2 = boxes[head];
    new_connection->distance = distance;

    connections[connections_idx] = new_connection;
    connections_idx++;

    if (head == line_count - 1) {
      tail++;
      head = tail + 1;
    } else {
      head++;
    }
  }

  qsort(connections, combinations, sizeof(Connection *), compare_connections);

  // example input uses 10 iterations
  int iterations = line_count < 30 ? 10 : 1000;
  int group_id = 0;
  for (size_t i = 0; i < iterations; i++) {
    int gid1 = connections[i]->box1[3];
    int gid2 = connections[i]->box2[3];

    if (gid1 == 0 && gid2 == 0) {
      group_id++;

      connections[i]->box1[3] = group_id;
      connections[i]->box2[3] = group_id;
    } else if (gid1 == gid2) {
      continue;
    } else if (gid1 == 0) {
      connections[i]->box1[3] = connections[i]->box2[3];
    } else if (gid2 == 0) {
      connections[i]->box2[3] = connections[i]->box1[3];
    } else {
      // union 1 consumes 2
      for (size_t j = 0; j < line_count; j++) {
        if (boxes[j][3] == gid2) {
          boxes[j][3] = gid1;
        }
      }
    }
  }

  size_t group_counts[line_count];
  for (size_t i = 0; i < line_count; i++) {
    group_counts[i] = 0;
  }

  for (size_t i = 0; i < line_count; i++) {
    group_counts[boxes[i][3]]++;
  }

  // ungrouped nodes do not count as a group
  group_counts[0] = 0;

  qsort(group_counts, line_count, sizeof(size_t), compare_sizet_desc);

  size_t pt_one = group_counts[0] * group_counts[1] * group_counts[2];
  printf("pt one: %zu\n", pt_one);

  // ------------------- pt two ---------------

  size_t box1;
  size_t box2;
  while (find_next_union_connection(boxes, line_count, &box1, &box2) == true) {
    int gid1 = boxes[box1][3];
    int gid2 = boxes[box2][3];

    // same union logic
    if (gid1 == 0 && gid2 == 0) {
      group_id++;

      boxes[box1][3] = group_id;
      boxes[box2][3] = group_id;
    } else if (gid1 == 0) {
      boxes[box1][3] = boxes[box2][3];
    } else if (gid2 == 0) {
      boxes[box2][3] = boxes[box1][3];
    } else {
      for (size_t j = 0; j < line_count; j++) {
        if (boxes[j][3] == gid2) {
          boxes[j][3] = gid1;
        }
      }
    }
  }

  printf("A: %d, %d, %d\n", boxes[box1][0], boxes[box1][1], boxes[box1][2]);
  printf("b: %d, %d, %d\n", boxes[box2][0], boxes[box2][1], boxes[box2][2]);

  size_t pt_two = boxes[box1][0] * boxes[box2][0];
  printf("pt two: %zu\n", pt_two);

  // ------------- cleanup -------------------
  for (size_t i = 0; i < combinations; i++) {
    free(connections[i]);
  }
  free(connections);
  for (size_t i = 0; i < line_count; i++) {
    free(boxes[i]);
  }
  free(boxes);
  return EXIT_SUCCESS;
}
