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

  const int deltas[8][2] = {{-1, 0}, {-1, 1}, {0, 1},  {1, 1},
                            {1, 0},  {1, -1}, {0, -1}, {-1, -1}};

  int pt_one = 0;
  for (int row = 0; row < num_rows; row++) {
    for (int col = 0; col < num_cols; col++) {
      if (graph[row][col] == '@') {
        int around = 0;

        for (int i = 0; i < 8; i++) {
          int n_row = row + deltas[i][0];
          int n_col = col + deltas[i][1];

          bool in_bounds = (n_row >= 0) && (n_col >= 0) && (n_row < num_rows) &&
                           (n_col < num_cols);

          if (in_bounds && graph[n_row][n_col] == '@') {
            around++;
          }
        }

        if (around < 4) {
          pt_one++;
        }
      }
    }
  }

  printf("pt one %d\n", pt_one);

  int pt_two = 0;
  bool last_round_removed = true;
  while (last_round_removed) {
    last_round_removed = false;

    int to_remove_idx = 0;
    // size is a guess here but seems okay
    int to_remove[pt_one * 2][2];

    // copy pasta because I am lazy
    for (int row = 0; row < num_rows; row++) {
      for (int col = 0; col < num_cols; col++) {
        if (graph[row][col] == '@') {
          int around = 0;

          for (int i = 0; i < 8; i++) {
            int n_row = row + deltas[i][0];
            int n_col = col + deltas[i][1];

            bool in_bounds = (n_row >= 0) && (n_col >= 0) &&
                             (n_row < num_rows) && (n_col < num_cols);

            if (in_bounds && graph[n_row][n_col] == '@') {
              around++;
            }
          }

          if (around < 4) {
            to_remove[to_remove_idx][0] = row;
            to_remove[to_remove_idx][1] = col;
            to_remove_idx++;
            pt_two++;
          }
        }
      }
    }

    if (to_remove_idx > 0) {
      last_round_removed++;
      for (int i = 0; i <= to_remove_idx; i++) {
        int row = to_remove[i][0];
        int col = to_remove[i][1];

        graph[row][col] = '.';
      }
    }
  }

  printf("pt two %d\n", pt_two);

  fclose(file);
  return EXIT_SUCCESS;
}
