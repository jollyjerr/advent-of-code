#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int value, index;
} SearchResult;

SearchResult joltage_search(char *line, int line_len, int start_idx) {
  int value = 0;
  int index = 0;

  for (int i = start_idx; i < line_len; i++) {
    int point = line[i] - '0';
    if (point > value) {
      value = point;
      index = i;
    }
  }

  SearchResult result = {value, index};
  return result;
}

long concat(long total, int value) {
  char total_buffer[40];
  char value_buffer[20];
  char result_str[60];

  sprintf(total_buffer, "%ld", total);
  sprintf(value_buffer, "%d", value);

  strcpy(result_str, total_buffer);
  strcat(result_str, value_buffer);

  return strtol(result_str, NULL, 10);
}

long calculate_joltage(char *line, int digits) {
  long total = 0;

  int line_len = strlen(line);
  SearchResult prev = joltage_search(line, line_len - digits, 0);

  total = concat(total, prev.value);

  for (int i = 1; i < digits; i++) {
    prev = joltage_search(line, line_len - (digits - i), prev.index + 1);
    total = concat(total, prev.value);
  }

  return total;
}

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed\n");
    return EXIT_FAILURE;
  }

  long pt_one = 0;
  long pt_two = 0;

  char line[256];
  while (fgets(line, 256, file) != NULL) {
    pt_one += calculate_joltage(line, 2);
    pt_two += calculate_joltage(line, 12);
  }

  printf("pt one: %ld\n", pt_one);
  printf("pt two: %ld\n", pt_two);

  fclose(file);
  return EXIT_SUCCESS;
}
