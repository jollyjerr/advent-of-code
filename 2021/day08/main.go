package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	partOne(data)
	partTwo(data)
}

func partTwo(data []segment) {
	count := 0
	for _, line := range data {
		display := Display{}
		one := findLen(line.input, 2)
		four := findLen(line.input, 4)
		seven := findLen(line.input, 3)
		eight := findLen(line.input, 7)

		if ret := difference(one, seven); len(ret) == 1 {
			display.t = ret[0]
		}

		var nine string
		for _, digit := range line.input {
			if len(digit) == 6 && len(difference(four+seven, digit)) == 1 {
				display.b = difference(four+seven, digit)[0]
				nine = digit
			}
		}

		if ret := difference(eight, nine); len(ret) == 1 {
			display.bl = ret[0]
		}

		var three string
		for _, digit := range line.input {
			if len(digit) == 5 && len(difference(digit, one+display.t+display.b)) == 1 {
				display.m = difference(digit, one+display.t+display.b)[0]
				three = digit
			}
		}

		if ret := difference(four, one+display.m); len(ret) == 1 {
			display.tl = ret[0]
		}

		zero := seven + display.b + display.tl + display.bl

		var six string
		for _, digit := range line.input {
			if len(digit) == 6 && len(difference(nine, digit)) > 1 && len(difference(zero, digit)) > 1 {
				six = digit
			}
		}

		if ret := difference(six, eight); len(ret) == 1 {
			display.tr = ret[0]
		}

		two := strings.Join([]string{display.t, display.tr, display.m, display.bl, display.b}, "")

		var five string
		for _, digit := range line.input {
			if len(digit) == 5 && len(difference(digit, three)) > 0 && len(difference(digit, two)) > 0 {
				five = digit
			}
		}

		options := make(map[string]string)
		for i, option := range []string{zero, one, two, three, four, five, six, seven, eight, nine} {
			options[option] = fmt.Sprint(i)
		}

		values := make([]string, 0)
		for _, symbol := range line.output {
			values = append(values, decode(symbol, options))
		}

		if num, err := strconv.Atoi(strings.Join(values, "")); err == nil {
			count += num
		}
	}
	fmt.Println("part two", count)
}

func decode(symbol string, options map[string]string) string {
	for key, option := range options {
		if len(difference(symbol, key)) == 0 {
			return option
		}
	}
	log.Fatal("noooo")
	return ""
}

func difference(a, b string) []string {
	aslice := strings.Split(a, "")
	bslice := strings.Split(b, "")
	ret := oneWayDifference(aslice, bslice)
	ret = append(ret, oneWayDifference(bslice, aslice)...)
	return ret
}

func oneWayDifference(a, b []string) []string {
	diff := make([]string, 0)
	seen := make(map[string]string)
	for _, digit := range a {
		seen[digit] = digit
	}
	for _, digit := range b {
		if _, ok := seen[digit]; !ok {
			diff = append(diff, digit)
		}
	}
	return diff
}

func findLen(data []string, target int) string {
	for _, char := range data {
		if len(char) == target {
			return char
		}
	}
	return ""
}

type Display struct {
	t  string
	tr string
	tl string
	m  string
	bl string
	b  string
}

func partOne(data []segment) {
	count := 0
	for _, seg := range data {
		for _, num := range seg.output {
			if len(num) == 2 || len(num) == 3 || len(num) == 4 || len(num) == 7 {
				count++
			}
		}
	}
	fmt.Println("part one", count)
}

type segment struct {
	input  []string
	output []string
}

func loadData() []segment {
	var data []segment
	if f, err := os.Open("./day8/data.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			line := strings.Split(scanner.Text(), " | ")
			newSegment := segment{
				input:  strings.Split(line[0], " "),
				output: strings.Split(line[1], " "),
			}
			data = append(data, newSegment)
		}
	}
	return data
}
