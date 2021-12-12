package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	data := loadData()
	seen := make([]string, 0)
	smallCaveException := "none"
	count := countPaths("start", "end", seen, data, smallCaveException)
	fmt.Println(count)
}

func countPaths(start, end string, seen []string, data map[string][]string, smallCaveException string) int {
	if start == end {
		return 1
	}
	if isSmallCave(start) && isInSlice(start, seen) {
		// sorta just writing part two on top of part one because it was just a change of rules
		if smallCaveException == "none" && !isInSlice(start, []string{"start", "end"}) {
			smallCaveException = start
		} else {
			return 0
		}
	}
	seen = append(seen, start)
	count := 0
	for _, item := range data[start] {
		count += countPaths(item, end, seen, data, smallCaveException)
	}
	return count
}

func isSmallCave(cave string) bool {
	return strings.ToLower(cave) == cave
}

func isInSlice(item string, list []string) bool {
	for _, test := range list {
		if item == test {
			return true
		}
	}
	return false
}

func loadData() map[string][]string {
	data := make(map[string][]string, 0)
	if f, err := os.Open("./day12/data.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			row := strings.Split(scanner.Text(), "-")

			// graph can be traversed in both directions
			data[row[0]] = append(data[row[0]], row[1])
			data[row[1]] = append(data[row[1]], row[0])
		}
	}
	return data
}
