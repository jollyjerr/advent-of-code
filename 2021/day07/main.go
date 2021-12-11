package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"

	"github.com/montanaflynn/stats"
)

func main() {
	data := loadData()
	partOne(data)
	partTwo(data)
}

func partTwo(data []int) {
	mean, _ := stats.Mean(stats.LoadRawData(data))
	targets := getPossibleTargets(mean)

	lowest := float64(9999999999)
	for _, target := range targets {
		cost := calculateCost(target, data)
		if cost < lowest {
			lowest = cost
		}
	}

	fmt.Println(lowest)
}

func getPossibleTargets(mean float64) []float64 {
	mid := math.Floor(mean)
	targets := make([]float64, 0)
	for i := (mid - 5); i < (mid + 5); i++ {
		if i >= 0 {
			targets = append(targets, i)
		}
	}
	return targets
}

func calculateCost(target float64, data []int) float64 {
	var cost float64
	for _, num := range data {
		moves := math.Abs(float64(num) - target)
		localCost := 0
		for i := 1; i < int(moves)+1; i++ {
			localCost += i
		}
		cost += float64(localCost)
	}
	return cost
}

func partOne(data []int) {
	median, _ := stats.Median(stats.LoadRawData(data))

	var fuel float64
	for _, num := range data {
		fuel += math.Abs(float64(num) - median)
	}

	fmt.Println(fuel)
}

func loadData() []int {
	var data []int
	if f, err := os.Open("./day7/data.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			nums := strings.Split(scanner.Text(), ",")
			for _, snum := range nums {
				num, _ := strconv.Atoi(snum)
				data = append(data, num)
			}
		}
	}
	return data
}
