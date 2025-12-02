#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

bool is_valid(long long value) {
    char strBuffer[30];
    sprintf(strBuffer, "%lld", value);
    int n = strlen(strBuffer);

    if (n % 2 != 0) {
        // all odd length numbers are valid I think?
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
  long long out;

  char *id_range = strtok(lineBuffer, ",");
  while (id_range != NULL) {
    if (sscanf(id_range, "%lld-%lld", &start, &end) == 2) {
        for (long long i = start; i <= end; i++) {
            if (!is_valid(i)) {
                out += i;
            }
        }
    }

    id_range = strtok(NULL, ",");
  }

  printf("pt one: %lld\n", out);

  fclose(file);
  return EXIT_SUCCESS;
}
