#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
  FILE *file = fopen(argv[1], "r");
  if (file == NULL) {
    perror("read data failed");
    return EXIT_FAILURE;
  }

  int num_problems = 0;
  int problems[1024][1024];
  int prob_num_lens[1024];
  int prob_number_idxs[1024];
  int prob_idx = 0;
  bool skip_spaces = true;

  char number[1024] = "";
  size_t num_len = 0;
  unsigned long long local_result = 0;

  unsigned long long pt_one = 0;

  int ch;
  while ((ch = fgetc(file)) != EOF) {
    switch (ch) {
    case '\n':
      if (!skip_spaces) {
        int number_idx = prob_number_idxs[prob_idx];

        num_len = strlen(number);
        if (num_len > prob_num_lens[prob_idx]) {
          prob_num_lens[prob_idx] = num_len;
        }

        problems[prob_idx][number_idx] = atoi(number);

        prob_number_idxs[prob_idx]++;
        prob_idx++;
        number[0] = '\0';

        skip_spaces = true;
      }
      prob_idx = 0;
      skip_spaces = true;
      break;
    case '+':
      for (int i = 0; i < prob_number_idxs[prob_idx]; i++) {
        if (local_result == 0) {
          local_result = problems[prob_idx][i];
        } else {
          local_result += problems[prob_idx][i];
        }
      }

      pt_one += local_result;
      local_result = 0;

      prob_idx++;
      num_problems++;
      break;
    case '*':
      for (int i = 0; i < prob_number_idxs[prob_idx]; i++) {
        if (local_result == 0) {
          local_result = problems[prob_idx][i];
        } else {
          local_result *= problems[prob_idx][i];
        }
      }

      pt_one += local_result;
      local_result = 0;

      prob_idx++;
      num_problems++;
      break;
    case ' ':
      if (!skip_spaces) {
        int number_idx = prob_number_idxs[prob_idx];

        num_len = strlen(number);
        if (num_len > prob_num_lens[prob_idx]) {
          prob_num_lens[prob_idx] = num_len;
        }

        problems[prob_idx][number_idx] = atoi(number);

        prob_number_idxs[prob_idx]++;
        prob_idx++;
        number[0] = '\0';

        skip_spaces = true;
      }
      break;
    default:
      num_len = strlen(number);

      number[num_len] = ch;
      number[num_len + 1] = '\0';

      skip_spaces = false;
      break;
    }
  }

  fclose(file);

  printf("pt one: %llu\n", pt_one);

  unsigned long long pt_two = 0;

  char local_number[1024];

  for (int i = 0; i < num_problems; i++) {
    int num_len = prob_num_lens[i];
    while (num_len > 0) {
      for (int j = 0; j < prob_number_idxs[i]; j++) {
        // big yikes...
        int val = problems[i][j];
        char str_val[1024];

        sprintf(str_val, "%d", val);
        printf("%c\n", str_val[num_len - 1]);

        local_number[j] = str_val[num_len];
      }

      // printf("%d\n", atoi(local_number));

      local_number[0] = '\0';
      num_len--;
    }
  }

  printf("pt two: %llu\n", pt_two);

  return EXIT_SUCCESS;
}
