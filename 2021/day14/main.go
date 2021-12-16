package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	data := loadData()
	fmt.Println(data)
}

func loadData() map[string][]string {
	data := make(map[string][]string)
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
