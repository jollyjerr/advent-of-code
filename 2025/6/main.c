#include <stdbool.h>
#include <stddef.h>
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

unsigned long solve_pt_one(char *worksheet) {
  unsigned long out = 0;

  int problems[1024][1024];
  int problem_idxs[1024];

  char number_buffer[1024] = "";
  size_t number_buffer_len = 0;
  bool skip_spaces = true;

  size_t prob_idx = 0;
  unsigned long prob_answer = 0;

  for (int i = 0; i < strlen(worksheet); i++) {
    int ch = worksheet[i];

    bool end_of_line = ch == '\n';
    bool end_of_number = (end_of_line || ch == ' ') && !skip_spaces;
    bool end_of_problem = (ch == '+' || ch == '*');
    bool is_int =
        !end_of_line && !end_of_number && !end_of_problem && (ch != ' ');

    if (end_of_number) {
      int number_idx = problem_idxs[prob_idx];

      problems[prob_idx][number_idx] = atoi(number_buffer);

      problem_idxs[prob_idx]++;
      prob_idx++;
      number_buffer[0] = '\0';

      skip_spaces = true;
    }

    if (end_of_line) {
      prob_idx = 0;
      skip_spaces = true;
    }

    if (end_of_problem) {
      for (int i = 0; i < problem_idxs[prob_idx]; i++) {
        if (prob_answer == 0) {
          prob_answer = problems[prob_idx][i];
        } else {
          if (ch == '+') {
            prob_answer += problems[prob_idx][i];
          } else {
            prob_answer *= problems[prob_idx][i];
          }
        }
      }

      out += prob_answer;
      prob_answer = 0;

      prob_idx++;
    }

    if (is_int) {
      number_buffer_len = strlen(number_buffer);

      number_buffer[number_buffer_len] = ch;
      number_buffer[number_buffer_len + 1] = '\0';

      skip_spaces = false;
    }
  }

  return out;
}

unsigned long long solve_pt_two(char *worksheet) {
  unsigned long long out = 0;

  int problem_count = 0;
  char *lines[6];
  size_t line_idx = 0;

  char *line = strtok(worksheet, "\n");
  while (line != NULL) {
    lines[line_idx] = line;

    line = strtok(NULL, "\n");
    line_idx++;
    problem_count++;
  }

  // read each column and save problem when meeting * or +
  int problem_numbers[8192];
  size_t prob_num_idx = 0;

  // backwards each column
  for (int i = strlen(lines[0]); i >= 0; i--) {
    char local_number[1024] = {};
    size_t local_number_idx = 0;

    bool should_add = false;
    bool should_mult = false;

    // for each row
    for (int j = 0; j < problem_count; j++) {
      int ch = lines[j][i];

      if (ch == ' ') {
        continue;
      }

      if (ch == '+') {
        should_add = true;
        continue;
      }

      if (ch == '*') {
        should_mult = true;
        continue;
      }

      local_number[local_number_idx] = ch;
      local_number_idx++;
    }

    if (strlen(local_number) > 0) {
      problem_numbers[prob_num_idx] = atoi(local_number);
      prob_num_idx++;
    }

    if (should_add || should_mult) {
      unsigned long long solution = problem_numbers[0];

      for (int k = 1; k < prob_num_idx; k++) {
        if (should_add) {
          solution += problem_numbers[k];
        } else {
          solution *= problem_numbers[k];
        }
      }

      out += solution;

      // clean memory for next problem
      for (int k = 0; k < prob_num_idx; k++) {
        problem_numbers[k] = 0;
      }
      prob_num_idx = 0;
    }
  }

  return out;
}

int main(int argc, char *argv[]) {
  char *worksheet = read_file(argv[1]);

  if (worksheet == NULL) {
    perror("no data");
    return EXIT_FAILURE;
  }

  printf("pt one: %lu\n", solve_pt_one(worksheet));
  printf("pt two: %llu\n", solve_pt_two(worksheet));

  free(worksheet);
  return EXIT_SUCCESS;
}
