#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int x, y;
} Coordinate;

typedef struct {
  Coordinate *data;
  int size;
  int capacity;
} CoordinateSet;

CoordinateSet *set_create() {
  CoordinateSet *set = malloc(sizeof(CoordinateSet));

  if (set == NULL) {
    fprintf(stderr, "Failed to allocate memory for CoordinateSet\n");
    return NULL;
  }

  set->capacity = 600;
  set->size = 0;
  set->data = malloc(set->capacity * sizeof(Coordinate));

  if (set->data == NULL) {
    fprintf(stderr, "Failed to allocate memory for coordinate data\n");
    free(set);
    return NULL;
  }

  return set;
}

bool set_contains(CoordinateSet *set, int x, int y) {
  for (int i = 0; i < set->size; i++) {
    if (set->data[i].x == x && set->data[i].y == y) {
      return true;
    }
  }
  return false;
}

void set_add(CoordinateSet *set, int x, int y) {
  if (set_contains(set, x, y)) {
    return;
  }

  if (set->size == set->capacity) {
    int old_capacity = set->capacity;
    set->capacity *= 2;

    printf("Resizing set from %d to %d\n", old_capacity, set->capacity);

    Coordinate *new_data =
        realloc(set->data, set->capacity * sizeof(Coordinate));
    if (new_data == NULL) {
      fprintf(stderr, "Failed to reallocate memory during resize\n");
      return;
    }
    set->data = new_data;
  }

  set->data[set->size].x = x;
  set->data[set->size].y = y;
  set->size++;
}

void set_free(CoordinateSet *set) {
  if (set == NULL) {
    printf("Attempted to free NULL set\n");
    return;
  }

  if (set->data != NULL) {
    free(set->data);
    set->data = NULL;
  }

  free(set);
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    return EXIT_FAILURE;
  }

  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("[ERROR] Failed to open file");
    return EXIT_FAILURE;
  }

  CoordinateSet *pt_one_coords = set_create();
  CoordinateSet *pt_two_coords = set_create();
  if (pt_one_coords == NULL || pt_two_coords == NULL) {
    set_free(pt_one_coords);
    set_free(pt_two_coords);
    fclose(file);
    return EXIT_FAILURE;
  }

  int pt_one_x = 0, pt_one_y = 0;
  set_add(pt_one_coords, pt_one_x, pt_one_y);

  int santa_x = 0, santa_y = 0;
  int robot_x = 0, robot_y = 0;
  set_add(pt_two_coords, santa_x, santa_y);
  set_add(pt_two_coords, robot_x, robot_y);

  bool santa_moves = true;
  int dr = 0, dc = 0;
  int ch;

  while ((ch = fgetc(file)) != EOF) {
    switch (ch) {
    case '^':
      dr = -1;
      dc = 0;
      break;
    case '>':
      dr = 0;
      dc = 1;
      break;
    case 'v':
      dr = 1;
      dc = 0;
      break;
    case '<':
      dr = 0;
      dc = -1;
      break;
    default:
      if (ch != '\n' && ch != '\r' && ch != ' ') {
        printf("Unmapped character: '%c' (ASCII: %d)\n", ch, ch);
      }
      continue;
    }

    pt_one_x = pt_one_x + dc;
    pt_one_y = pt_one_y + dr;
    set_add(pt_one_coords, pt_one_x, pt_one_y);

    if (santa_moves == true) {
      santa_x = santa_x + dc;
      santa_y = santa_y + dr;
      set_add(pt_two_coords, santa_x, santa_y);

      santa_moves = false;
    } else {
      robot_x = robot_x + dc;
      robot_y = robot_y + dr;
      set_add(pt_two_coords, robot_x, robot_y);

      santa_moves = true;
    }
  }

  printf("\nPart One: number of houses with at least one present: %d\n",
         pt_one_coords->size);
  printf("\nPart Two: number of houses with at least one present: %d\n",
         pt_two_coords->size);

  set_free(pt_one_coords);
  set_free(pt_two_coords);
  fclose(file);

  return EXIT_SUCCESS;
}
