#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed\n");
    return EXIT_FAILURE;
  }

  int num_rows = 0;
  int num_cols = 0;
  char graph[140][140];

  char line[140];
  while (fgets(line, 142, file) != NULL) {
    int n = strlen(line);

    if (num_rows == 0) {
      num_cols = n - 1;
    }

    for (int i = 0; i < (n - 1); i++) {
      graph[num_rows][i] = line[i];
    }

    num_rows++;
  }

  int pt_one = 0;
  for (int row = 0; row < num_rows; row++) {
    for (int col = 0; col < num_cols; col++) {
      if (graph[row][col] == '@') {
        int around = 0;

        int up = col - 1;
        int right = row + 1;
        int down = col + 1;
        int left = row - 1;
        bool can_up = up >= 0;
        bool can_right = right < num_cols;
        bool can_down = down < num_rows;
        bool can_left = left >= 0;

        // up
        if (can_up && graph[row][up] == '@') {
          around++;
        }
        // up-right
        if (can_up && can_right && graph[right][up] == '@') {
          around++;
        }
        // right
        if (can_right && graph[right][col] == '@') {
          around++;
        }
        // down-right
        if (can_down && can_right && graph[right][down] == '@') {
          around++;
        }
        // down
        if (can_down && graph[row][down] == '@') {
          around++;
        }
        // down-left
        if (can_down && can_left && graph[left][down] == '@') {
          around++;
        }
        // left
        if (can_left && graph[left][col] == '@') {
          around++;
        }
        // up-left
        if (can_up && can_left && graph[left][up] == '@') {
          around++;
        }

        if (around < 4) {
          pt_one++;
        }
      }
    }
  }

  printf("pt one %d\n", pt_one);

  fclose(file);
  return EXIT_SUCCESS;
}
