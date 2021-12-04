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

	var gamma []string
	var epsilon []string
	for i := 0; i < len(data[0]); i++ {
		count1 := 0
		count2 := 0
		for j := 0; j < len(data); j++ {
			if string(data[j][i]) == "1" {
				count1++
			} else {
				count2++
			}
		}
		if count1 > count2 {
			gamma = append(gamma, "1")
			epsilon = append(epsilon, "0")
		} else {
			gamma = append(gamma, "0")
			epsilon = append(epsilon, "1")
		}
	}

	gamdec, _ := strconv.ParseInt(strings.Join(gamma, ""), 2, 64)
	epdec, _ := strconv.ParseInt(strings.Join(epsilon, ""), 2, 64)

	fmt.Println(gamdec * epdec)
}

func loadData() []string {
	var data []string
	f, err := os.Open("./day3/data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return data
}
