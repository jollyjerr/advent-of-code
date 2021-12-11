package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	numSteps := 600
	data := loadData()

	flashes := 0
	allFlashedOnSteps := make([]int, 0)
	for i := 0; i < numSteps; i++ {
		newFlashes, allFlashed := step(data)
		flashes += newFlashes

		if allFlashed {
			allFlashedOnSteps = append(allFlashedOnSteps, i+1)
		}
	}
	fmt.Println("Part One: Had", flashes, "flashes in", numSteps, "steps.")
	fmt.Println("Part Two: They all flashed first on step", allFlashedOnSteps[0])
}

func step(data map[string]octopus) (int, bool) {
	// First, the energy level of each octopus increases by 1
	for _, octo := range data {
		copy := data[octo.id]
		copy.energyLevel += 1
		data[octo.id] = copy
	}

	// Then, any octopus with an energy level greater than 9 flashes
	for _, octo := range data {
		commenceFlashing(data, octo.id)
	}

	// Finally, any octopus that flashed during this step has its energy level set to 0
	flashes := 0
	allFlashed := false
	for _, octo := range data {
		if octo.flashed {
			flashes++
			copy := data[octo.id]
			copy.flashed = false // ??
			copy.energyLevel = 0
			data[octo.id] = copy
		}
	}

	// lazy hard code
	if flashes == 100 {
		allFlashed = true
	}

	return flashes, allFlashed
}

func commenceFlashing(data map[string]octopus, key string) {
	if data[key].energyLevel > 9 && !data[key].flashed {
		setFlashed(data, key)
		for _, newKey := range data[key].edges {
			copy := data[newKey]
			copy.energyLevel += 1
			data[newKey] = copy
			commenceFlashing(data, newKey)
		}
	}
}

func setFlashed(data map[string]octopus, key string) {
	copy := data[key]
	copy.flashed = true
	data[key] = copy
}

type octopus struct {
	id          string
	energyLevel int
	flashed     bool
	edges       []string
}

func loadData() map[string]octopus {
	var data [][]octopus

	if f, err := os.Open("./day11/data.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		ycount := 0
		for scanner.Scan() {
			row := make([]octopus, 0)
			xcount := 0
			for _, val := range scanner.Text() {
				if num, err := strconv.Atoi(string(val)); err == nil {
					newOcti := octopus{
						id:          getOctoId(xcount, ycount),
						energyLevel: num,
						flashed:     false,
					}
					xcount++
					row = append(row, newOcti)
				}
			}
			ycount++
			data = append(data, row)
		}
	}

	octopi := make(map[string]octopus, 0)
	for y, row := range data {
		for x, octo := range row {
			if y == 0 {
				if x == 0 {
					octo.edges = []string{r(data, x, y), b(data, x, y), br(data, x, y)}
				} else if x == len(row)-1 {
					octo.edges = []string{b(data, x, y), bl(data, x, y), l(data, x, y)}
				} else {
					octo.edges = []string{l(data, x, y), r(data, x, y), bl(data, x, y), b(data, x, y), br(data, x, y)}
				}
			} else if x == 0 {
				if y == len(data)-1 {
					octo.edges = []string{u(data, x, y), ur(data, x, y), r(data, x, y)}
				} else {
					octo.edges = []string{u(data, x, y), ur(data, x, y), r(data, x, y), br(data, x, y), b(data, x, y)}
				}
			} else if y == len(data)-1 {
				if x == len(row)-1 {
					octo.edges = []string{u(data, x, y), ul(data, x, y), l(data, x, y)}
				} else {
					octo.edges = []string{l(data, x, y), ul(data, x, y), u(data, x, y), ur(data, x, y), r(data, x, y)}
				}
			} else if x == len(row)-1 {
				octo.edges = []string{u(data, x, y), ul(data, x, y), l(data, x, y), bl(data, x, y), b(data, x, y)}
			} else {
				octo.edges = []string{ul(data, x, y), u(data, x, y), ur(data, x, y), l(data, x, y), r(data, x, y), bl(data, x, y), b(data, x, y), br(data, x, y)}
			}
			octopi[octo.id] = octo
		}
	}

	return octopi
}

func getOctoId(x, y int) string {
	return fmt.Sprintf("%v,%v", x, y)
}

func ul(data [][]octopus, x, y int) string {
	return data[y-1][x-1].id
}
func u(data [][]octopus, x, y int) string {
	return data[y-1][x].id
}
func ur(data [][]octopus, x, y int) string {
	return data[y-1][x+1].id
}
func l(data [][]octopus, x, y int) string {
	return data[y][x-1].id
}
func r(data [][]octopus, x, y int) string {
	return data[y][x+1].id
}
func bl(data [][]octopus, x, y int) string {
	return data[y+1][x-1].id
}
func b(data [][]octopus, x, y int) string {
	return data[y+1][x].id
}
func br(data [][]octopus, x, y int) string {
	return data[y+1][x+1].id
}
