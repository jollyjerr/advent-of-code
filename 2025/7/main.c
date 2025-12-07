#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *read_file(const char *filename) {
  FILE *file = fopen(filename, "r");

  if (file == NULL) {
    perror("read data failed");
    return NULL;
  }

  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  rewind(file);

  char *contents = (char *)malloc(file_size + 1);
  fread(contents, 1, file_size, file);

  contents[file_size] = '\0';

  fclose(file);
  return contents;
}

int main(int argc, char *argv[]) {
  char *tachyon_diagram = read_file(argv[1]);

  if (tachyon_diagram == NULL) {
    perror("no data");
    return EXIT_FAILURE;
  }

  size_t row_count = 0;
  for (int i = 0; i < strlen(tachyon_diagram); i++) {
    if (tachyon_diagram[i] == '\n') {
      row_count++;
    }
  }

  char *grid[row_count];
  size_t grid_idx = 0;

  char *row = strtok(tachyon_diagram, "\n");
  while (row != NULL) {
    grid[grid_idx] = row;

    row = strtok(NULL, "\n");
    grid_idx++;
  }

  unsigned long pt_one = 0;

  for (int row = 0; row < row_count; row++) {
    for (int col = 0; col < strlen(grid[row]); col++) {
      int ch = grid[row][col];

      if (ch == 'S') {
        grid[row + 1][col] = '|';
      } else if (ch == '|') {
        if (grid[row + 1][col] == '.') {
          grid[row + 1][col] = '|';
        } else if (grid[row + 1][col] == '^') {
          pt_one++;
          grid[row + 1][col - 1] = '|';
          grid[row + 1][col + 1] = '|';
        }
      }
    }
  }

  printf("pt one: %lu\n", pt_one);

  return EXIT_SUCCESS;
}
