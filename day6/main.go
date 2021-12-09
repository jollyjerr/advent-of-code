package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	runSimulation(data, 256)
}

func runSimulation(data []int, days int) {
	fishes := make(map[int]int)
	for _, fish := range data {
		fishes[fish] += 1
	}
	// for each day
	for i := 0; i < days; i++ {
		babies := fishes[0]
		for i := 1; i < 9; i++ {
			fishes[i-1] = fishes[i]
		}
		fishes[8] = babies
		fishes[6] += babies
	}

	count := 0
	for _, val := range fishes {
		count += val
	}

	fmt.Println(count)
}

func loadData() []int {
	var data []int
	f, _ := os.Open("./day6/data.txt")
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		nums := strings.Split(scanner.Text(), ",")
		for _, char := range nums {
			num, _ := strconv.Atoi(char)
			data = append(data, num)
		}
	}
	return data
}
