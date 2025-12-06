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

unsigned long long solve_pt_one(char *worksheet) {
  unsigned long long out = 0;

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

int main(int argc, char *argv[]) {
  char *worksheet = read_file(argv[1]);

  if (worksheet == NULL) {
    perror("no data");
    return EXIT_FAILURE;
  }

  printf("pt one: %llu\n", solve_pt_one(worksheet));

  free(worksheet);
  return EXIT_SUCCESS;
}
