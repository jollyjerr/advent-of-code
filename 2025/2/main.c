#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

bool is_valid_pt_one(long long value) {
  char strBuffer[30];
  sprintf(strBuffer, "%lld", value);
  int n = strlen(strBuffer);

  // all odd length numbers are valid
  if (n % 2 != 0) {
    return true;
  }

  int tail = 0;
  int head = n / 2;

  for (int i = 0; i < n / 2; i++) {
    if (strBuffer[tail] != strBuffer[head]) {
      return true;
    }
    tail++;
    head++;
  }

  return false;
}

bool is_valid_pt_two(long long value) {
  char strBuffer[30];
  sprintf(strBuffer, "%lld", value);
  int n = strlen(strBuffer);

  char sub_str[30];

  // n - 1 so we only check substrings
  for (int i = 0; i < n - 1; i++) {
    // cannot be pattern mathematically
    if (n % (i + 1) != 0) {
      continue;
    }

    int sub_n = i + 1;

    // get substring
    strncpy(sub_str, strBuffer, sub_n);
    sub_str[i + 1] = '\0';

    // repeat substring until length of original string
    char check[n + 1];
    check[0] = '\0';

    int loops = n / sub_n;
    for (int j = 0; j < loops; j++) {
      strcat(check, sub_str);
    }

    // check if identical
    if (strcmp(check, strBuffer) == 0) {
      return false;
    }
  }

  return true;
}

int main(int argc, char *argv[]) {
  char *data_path = argv[1];

  FILE *file = fopen(data_path, "r");
  if (file == NULL) {
    perror("read data failed\n");
    return EXIT_FAILURE;
  }

  char lineBuffer[1240];
  fgets(lineBuffer, sizeof(lineBuffer), file);

  long long start;
  long long end;

  long long pt_one = 0;
  long long pt_two = 0;

  char *id_range = strtok(lineBuffer, ",");
  while (id_range != NULL) {
    if (sscanf(id_range, "%lld-%lld", &start, &end) == 2) {
      for (long long i = start; i <= end; i++) {
        if (!is_valid_pt_one(i)) {
          pt_one += i;
        }
        if (!is_valid_pt_two(i)) {
          pt_two += i;
        }
      }
    }

    id_range = strtok(NULL, ",");
  }

  printf("pt one: %lld\n", pt_one);
  printf("pt two: %lld\n", pt_two);

  fclose(file);
  return EXIT_SUCCESS;
}
