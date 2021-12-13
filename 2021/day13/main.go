package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	points, instructions := loadData()
	highest := make([]int, 2)
	for _, p := range points {
		if p[0] > highest[0] {
			highest[0] = p[0]
		}
		if p[1] > highest[1] {
			highest[1] = p[1]
		}
	}

	for _, point := range points {
		fold(point, instructions[0])
	}
	fmt.Println("Part one", countPoints(points))

	for i := 1; i < len(instructions); i++ {
		for _, point := range points {
			fold(point, instructions[i])
		}
	}

	grid := make([][]int, highest[0])
	for i := 0; i < len(grid); i++ {
		grid[i] = make([]int, highest[0])
	}
	for _, point := range points {
		if grid[point[1]][point[0]] == 0 {
			grid[point[1]][point[0]] = 1
		}
	}
	fmt.Println("Part two :)")
	for _, line := range grid {
		fmt.Println(line)
	}
}

func fold(point, instruction []int) {
	if instruction[0] > 0 {
		// fold x
		if point[0] > instruction[0] {
			diff := point[0] - instruction[0]
			point[0] = instruction[0] - diff
		}
	} else {
		// fold y
		if point[1] > instruction[1] {
			diff := point[1] - instruction[1]
			point[1] = instruction[1] - diff
		}
	}
}

func countPoints(points [][]int) int {
	seen := make([]string, 0)

	for _, point := range points {
		candidate := sliceToString(point)
		if !isInSlice(candidate, seen) {
			seen = append(seen, candidate)
		}
	}

	return len(seen)
}

func sliceToString(points []int) string {
	return fmt.Sprintf("%v,%v", points[0], points[1])
}

func isInSlice(item string, list []string) bool {
	for _, test := range list {
		if item == test {
			return true
		}
	}
	return false
}

// going to try data as [[x,y],[x,y]]
func loadData() ([][]int, [][]int) {
	allPoints := make([][]int, 0)
	instructions := make([][]int, 0)
	if f, err := os.Open("./day13/data.txt"); err == nil {
		scanningPoints := true
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			if scanner.Text() == "" {
				scanningPoints = false
				continue
			}
			if scanningPoints {
				points := make([]int, 0)
				stringpoints := strings.Split(scanner.Text(), ",")
				for _, point := range stringpoints {
					if num, err := strconv.Atoi(point); err == nil {
						points = append(points, num)
					}
				}
				allPoints = append(allPoints, points)
			} else {
				details := strings.Split(scanner.Text(), "=")
				if num, err := strconv.Atoi(details[1]); err == nil {
					// no instructions say to fold by a value of 0
					if strings.Contains(details[0], "x") {
						instructions = append(instructions, []int{num, 0})
					} else {
						instructions = append(instructions, []int{0, num})
					}
				}
			}
		}
	}
	return allPoints, instructions
}
