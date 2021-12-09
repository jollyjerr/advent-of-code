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
	partOne(data)
	partTwo(data)
}

func partOne(data []Vent) {
	points := make(map[int]map[int]int)

	for _, vent := range data {
		// part one excludes diagonals
		if !vent.diagonal {

			for i := 0; i < len(vent.horizontals); i++ {
				incrementPoint(points, vent, i)
			}

		}
	}

	count := 0
	for _, xval := range points {
		for _, yval := range xval {
			if yval > 1 {
				count++
			}
		}
	}

	fmt.Println("Part 1", count)
}

func partTwo(data []Vent) {
	points := make(map[int]map[int]int)

	for _, vent := range data {
		for i := 0; i < len(vent.horizontals); i++ {
			incrementPoint(points, vent, i)
		}
	}

	count := 0
	for _, xval := range points {
		for _, yval := range xval {
			if yval > 1 {
				count++
			}
		}
	}

	fmt.Println("Part 2", count)
}

func incrementPoint(points map[int]map[int]int, vent Vent, i int) {
	if _, ok := points[vent.horizontals[i]]; ok {
		points[vent.horizontals[i]][vent.verticals[i]] += 1
	} else {
		points[vent.horizontals[i]] = make(map[int]int)
		points[vent.horizontals[i]][vent.verticals[i]] += 1
	}
}

type Vent struct {
	horizontals []int
	verticals   []int
	diagonal    bool
}

func loadData() []Vent {
	var ret []Vent
	f, _ := os.Open("./day5/data.txt")
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		newVent := Vent{
			horizontals: make([]int, 0),
			verticals:   make([]int, 0),
			diagonal:    false,
		}

		// read and parse to ints
		cordsstr := strings.Split(scanner.Text(), "->")
		startstr := strings.Split(cordsstr[0], ",")
		endstr := strings.Split(cordsstr[1], ",")
		start := make([]int, 0)
		end := make([]int, 0)
		for _, val := range startstr {
			realVal := strings.ReplaceAll(val, " ", "")
			num, _ := strconv.Atoi(realVal)
			start = append(start, num)
		}
		for _, val := range endstr {
			realVal := strings.ReplaceAll(val, " ", "")
			num, _ := strconv.Atoi(realVal)
			end = append(end, num)
		}

		// fill in all points covered
		if start[0] < end[0] {
			for i := start[0]; i <= end[0]; i++ {
				newVent.horizontals = append(newVent.horizontals, i)
			}
		} else {
			for i := start[0]; i >= end[0]; i-- {
				newVent.horizontals = append(newVent.horizontals, i)
			}
		}
		if start[1] < end[1] {
			for i := start[1]; i <= end[1]; i++ {
				newVent.verticals = append(newVent.verticals, i)
			}
		} else {
			for i := start[1]; i >= end[1]; i-- {
				newVent.verticals = append(newVent.verticals, i)
			}
		}

		// fill in static cords to so you can assume all pairs are filled
		if len(newVent.verticals) > len(newVent.horizontals) {
			for i := 1; i < len(newVent.verticals); i++ {
				newVent.horizontals = append(newVent.horizontals, newVent.horizontals[0])
			}
		} else if len(newVent.horizontals) > len(newVent.verticals) {
			for i := 1; i < len(newVent.horizontals); i++ {
				newVent.verticals = append(newVent.verticals, newVent.verticals[0])
			}
		}

		// mark if diagonal line
		if (start[0] != end[0]) && (start[1] != end[1]) {
			newVent.diagonal = true
		}

		ret = append(ret, newVent)
	}
	return ret
}
