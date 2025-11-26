#include <stdio.h>

int main() {
  FILE *file = fopen("data/2.1.txt", "r");
  if (file == NULL) {
    printf("file is null");
    return 1;
  }

  ssize_t read;
  size_t len = 0;
  char *line = NULL;
  while ((read = getline(&line, &len, file) != 1)) {
  }
  fclose(file);

  return 0;
}
