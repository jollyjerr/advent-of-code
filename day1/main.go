package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func loadData() []int {
	var data []int
	f, err := os.Open("./day1/data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		num, _ := strconv.Atoi(scanner.Text())
		data = append(data, num)
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return data
}

func main() {
	data := loadData()

	count := 0
	for i, num := range data {
		if i > 0 {
			if num > data[i-1] {
				count++
			}
		}
	}
	fmt.Println("Part 1:", count)

	var prev int
	var current int
	windowCount := 0
	for i := 2; i < len(data); i++ {
		current = data[i-2] + data[i-1] + data[i]
		if i > 2 {
			if current > prev {
				windowCount++
			}
		}
		prev = current
	}
	fmt.Println("Part 2:", windowCount)
}
