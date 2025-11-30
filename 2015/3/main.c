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

  CoordinateSet *coords = set_create();
  if (coords == NULL) {
    fclose(file);
    return EXIT_FAILURE;
  }

  int x = 0, y = 0;
  set_add(coords, x, y);

  int dr = 0, dc = 0;
  int ch;
  int move_count = 0;

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

    x = x + dc;
    y = y + dr;
    move_count++;
    set_add(coords, x, y);
  }

  printf("\nNumber of houses with at least one present: %d\n", coords->size);

  set_free(coords);
  fclose(file);

  return EXIT_SUCCESS;
}
