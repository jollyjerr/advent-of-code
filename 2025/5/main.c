#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int range_by_lowest_low(const void *a, const void *b) {
  const unsigned long long (*range_a)[2] = a;
  const unsigned long long (*range_b)[2] = b;

  unsigned long long low_a = (*range_a)[0];
  unsigned long long low_b = (*range_b)[0];

  if (low_a < low_b) {
    return -1;
  } else if (low_a > low_b) {
    return 1;
  } else {
    return 0;
  }
}

int main(int argc, char *argv[]) {
  FILE *file = fopen(argv[1], "r");
  if (file == NULL) {
    perror("read data failed");
    return EXIT_FAILURE;
  }

  int pt_one = 0;

  unsigned long long ranges[256][2];
  int range_count = 0;
  bool parsing_ranges = true;

  char line[256];
  while (fgets(line, 256, file) != NULL) {
    line[strcspn(line, "\n")] = 0;

    if (parsing_ranges && strlen(line) == 0) {
      parsing_ranges = false;
      continue;
    }

    if (parsing_ranges) {
      unsigned long long low;
      unsigned long long high;

      if (sscanf(line, "%llu-%llu", &low, &high) == 2) {
        ranges[range_count][0] = low;
        ranges[range_count][1] = high;
        range_count++;
      }
    } else {
      char *endpntr;
      unsigned long long val = strtoull(line, &endpntr, 10);

      if (endpntr == line) {
        printf("error reading val go fix :(\n");
        continue;
      }

      for (int j = 0; j < range_count; j++) {
        if (ranges[j][0] <= val && val <= ranges[j][1]) {
          pt_one++;
          break;
        }
      }
    }
  }

  // 631 low;
  printf("pt one: %d\n", pt_one);

  qsort(ranges, range_count, sizeof(ranges[0]), range_by_lowest_low);

  int tail = 0;
  int head = 1;
  while (true) {
    if (head >= range_count) {
      break;
    }

    if (ranges[tail][1] >= ranges[head][0] - 1) {
      if (ranges[tail][1] < ranges[head][1]) {
        ranges[tail][1] = ranges[head][1];
      }
    } else {
      tail++;

      if (tail != head) {
        ranges[tail][0] = ranges[head][0];
        ranges[tail][1] = ranges[head][1];
      }
    }

    head++;
  }

  unsigned long long pt_two = 0;
  for (int j = 0; j <= tail; j++) {
    pt_two += (ranges[j][1] - ranges[j][0]) + 1;
  }

  // 6975633923672 low; 426000547810882 high; 335666363781475 low;
  printf("pt two: %llu\n", pt_two);

  fclose(file);
  return EXIT_SUCCESS;
}
