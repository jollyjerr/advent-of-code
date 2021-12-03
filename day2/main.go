package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Ship struct {
	horizontal int
	depth      int
}

func (ship *Ship) forward(units int) {
	ship.horizontal += units
}
func (ship *Ship) down(units int) {
	ship.depth += units
}
func (ship *Ship) up(units int) {
	ship.depth -= units
}

type BetterShip struct {
	horizontal int
	depth      int
	aim        int
}

func (ship *BetterShip) down(units int) {
	ship.aim += units
}
func (ship *BetterShip) up(units int) {
	ship.aim -= units
}
func (ship *BetterShip) forward(units int) {
	ship.horizontal += units
	ship.depth += (ship.aim * units)
}

func main() {
	data := loadData()
	ship := Ship{
		horizontal: 0,
		depth:      0,
	}
	betterShip := BetterShip{
		horizontal: 0,
		depth:      0,
		aim:        0,
	}

	for _, instruction := range data {
		switch instruction.direction {
		case "forward":
			ship.forward(instruction.units)
			betterShip.forward(instruction.units)
		case "down":
			ship.down(instruction.units)
			betterShip.down(instruction.units)
		case "up":
			ship.up(instruction.units)
			betterShip.up(instruction.units)
		}
	}

	fmt.Println("Part 1:", ship.horizontal*ship.depth)
	fmt.Println("Part 2:", betterShip.horizontal*betterShip.depth)
}

type Instruction struct {
	direction string
	units     int
}

func loadData() []Instruction {
	var data []Instruction
	f, err := os.Open("./day2/data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		num, _ := strconv.Atoi(fields[1])
		data = append(data, Instruction{
			direction: fields[0],
			units:     num,
		})
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return data
}
